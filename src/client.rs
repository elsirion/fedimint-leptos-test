use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use std::time::{Duration, SystemTime};

use fedimint_client::secret::{PlainRootSecretStrategy, RootSecretStrategy};
use fedimint_client::Client;
use fedimint_core::api::InviteCode;
use fedimint_core::config::ClientConfig;
use fedimint_core::core::OperationId;
use fedimint_core::db::{Database, IDatabaseTransactionOpsCore, IRawDatabase};
use fedimint_core::task::spawn;
use fedimint_core::util::BoxStream;
use fedimint_core::Amount;
use fedimint_ln_client::{
    LightningClientInit, LightningClientModule, LightningOperationMeta, LightningOperationMetaPay,
    LightningOperationMetaVariant,
};
use fedimint_mint_client::{
    MintClientInit, MintClientModule, MintOperationMeta, MintOperationMetaVariant, OOBNotes,
};
use fedimint_wallet_client::WalletClientInit;
use futures::{StreamExt, TryFutureExt};
use leptos::logging::warn;
use lightning_invoice::{Bolt11Invoice, Bolt11InvoiceDescription};
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;
use tokio::sync::{mpsc, oneshot, watch};
use tracing::{debug, info};

use crate::db::PersistentMemDb;

#[derive(Debug, Clone)]
enum RpcRequest {
    ListWallets,
    SelectWallet(String),
    Join(String),
    GetName,
    SubscribeBalance,
    EcashSend(Amount),
    EcashReceive(String),
    LnSend(String),
    LnReceive { amount: Amount, description: String },
    // TODO: pagination
    ListTransactions,
}

enum RpcResponse {
    ListWallets(Vec<String>),
    SelectWallet {
        initialized: bool,
    },
    Join,
    GetName(String),
    SubscribeBalance(BoxStream<'static, Amount>),
    EcashSend(OOBNotes),
    EcashReceive(Amount),
    LnSend,
    LnReceive {
        invoice: String,
        await_paid: watch::Receiver<bool>,
    },
    ListTransactions(Vec<Transaction>),
}

// TODO: add status update stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub timestamp: SystemTime,
    pub operation_id: OperationId,
    pub operation_kind: String,
    pub amount_msat: i64,
    pub description: Option<String>,
}

impl Debug for RpcResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RpcResponse::?")
    }
}

#[derive(Debug, ThisError, Serialize, Deserialize, Clone)]
pub enum RpcError {
    #[error("Invalid response")]
    InvalidResponse,
    #[error("Client stopped")]
    ClientStopped(String),
}

impl From<anyhow::Error> for RpcError {
    fn from(e: anyhow::Error) -> Self {
        Self::ClientStopped(e.to_string())
    }
}

type RpcCall = (RpcRequest, oneshot::Sender<anyhow::Result<RpcResponse>>);

// TODO: use bip39 and proper derivation
async fn load_or_generate_entropy(db: &Database) -> [u8; 64] {
    if let Ok(entropy) = Client::load_decodable_client_secret::<[u8; 64]>(db).await {
        entropy
    } else {
        info!("Generating mnemonic and writing entropy to client storage");
        let entropy = PlainRootSecretStrategy::random(&mut thread_rng());
        Client::store_encodable_client_secret(db, entropy)
            .await
            .expect("DB error");
        entropy
    }
}

async fn run_client(mut rpc: mpsc::Receiver<RpcCall>) {
    // Open DB
    let (wallet_db, joined) = loop {
        let (wallet_db_name, response_sender) = match rpc.recv().await.expect("Sender not dropped")
        {
            (RpcRequest::SelectWallet(wallet_db_name), response_sender) => {
                (wallet_db_name, response_sender)
            }
            (RpcRequest::ListWallets, response_sender) => {
                let _ = response_sender
                    .send(Ok(RpcResponse::ListWallets(PersistentMemDb::list_dbs())))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
                continue;
            }
            (_, response_sender) => {
                let _ = response_sender
                    .send(Err(anyhow::anyhow!(
                        "Invalid request, need to initialize client first"
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
                continue;
            }
        };

        info!("Opening Wallet DB {}", wallet_db_name);
        let wallet_db = PersistentMemDb::new(wallet_db_name).await;
        let joined = {
            let mut dbtx = wallet_db.begin_transaction().await;
            let mut stream = dbtx.raw_find_by_prefix(&[0x2f]).await.expect("DB error");
            stream.next().await.is_some()
        };

        let _ = response_sender
            .send(Ok(RpcResponse::SelectWallet {
                initialized: joined,
            }))
            .map_err(|_| warn!("RPC receiver dropped before response was sent"));

        break (wallet_db, joined);
    };

    let client = if !joined {
        loop {
            let (invite_code_str, response_sender) =
                match rpc.recv().await.expect("Sender not dropped") {
                    (RpcRequest::Join(invite_code_str), response_sender) => {
                        (invite_code_str, response_sender)
                    }
                    (_, response_sender) => {
                        let _ = response_sender
                            .send(Err(anyhow::anyhow!(
                                "Invalid request, need to initialize client first"
                            )))
                            .map_err(|_| warn!("RPC receiver dropped before response was sent"));
                        continue;
                    }
                };

            info!("Joining federation {}", invite_code_str);

            let invite_code = match InviteCode::from_str(&invite_code_str) {
                Ok(invite) => invite,
                Err(e) => {
                    let _ = response_sender
                        .send(Err(anyhow::anyhow!("Invalid invite code: {e:?}")))
                        .map_err(|_| warn!("RPC receiver dropped before response was sent"));
                    continue;
                }
            };

            let mut client_builder = fedimint_client::Client::builder(wallet_db.clone().into());

            let client_secret = load_or_generate_entropy(client_builder.db()).await;

            client_builder.with_module(WalletClientInit(None));
            client_builder.with_module(MintClientInit);
            client_builder.with_module(LightningClientInit);
            client_builder.with_primary_module(1);
            let client_res = ClientConfig::download_from_invite_code(&invite_code)
                .and_then(|cfg| {
                    client_builder
                        .join(PlainRootSecretStrategy::to_root_secret(&client_secret), cfg)
                })
                .await;

            match client_res {
                Ok(client) => {
                    let _ = response_sender
                        .send(Ok(RpcResponse::Join))
                        .map_err(|_| warn!("RPC receiver dropped before response was sent"));
                    break client;
                }
                Err(e) => {
                    let _ = response_sender
                        .send(Err(anyhow::anyhow!("Failed to initialize client: {e:?}")))
                        .map_err(|_| warn!("RPC receiver dropped before response was sent"));
                    continue;
                }
            };
        }
    } else {
        // TODO: dedup
        let mut client_builder = fedimint_client::Client::builder(wallet_db.into());

        let client_secret = load_or_generate_entropy(client_builder.db()).await;

        client_builder.with_module(WalletClientInit(None));
        client_builder.with_module(MintClientInit);
        client_builder.with_module(LightningClientInit);
        client_builder.with_primary_module(1);
        client_builder
            .open(PlainRootSecretStrategy::to_root_secret(&client_secret))
            .await
            .unwrap()
    };

    let client: &Client = Box::leak(Box::new(client));

    info!("Client initialized");

    let meta_service = client.meta_service().clone();
    let db = client.db().clone();
    spawn("gateway updater", async {
        client
            .get_first_module::<LightningClientModule>()
            .update_gateway_cache_continuously(move |gws| {
                let db_inner = db.clone();
                let meta_service_inner = meta_service.clone();
                async move {
                    let vetted_gateways = meta_service_inner
                        .get_field::<Vec<secp256k1_zkp::PublicKey>>(&db_inner, "vetted_gateways")
                        .await
                        .and_then(|meta_field| meta_field.value)
                        .unwrap_or_default();
                    gws.into_iter()
                        .filter(|gw| vetted_gateways.contains(&gw.info.gateway_id))
                        .collect()
                }
            })
            .await
    });

    info!("Started gateway update service");

    while let Some((rpc_request, response_sender)) = rpc.recv().await {
        debug!("Received RPC request: {:?}", rpc_request);
        match rpc_request {
            RpcRequest::GetName => {
                let name = client
                    .get_meta("federation_name")
                    .unwrap_or("<unknown>".to_string());
                let _ = response_sender
                    .send(Ok(RpcResponse::GetName(name)))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            RpcRequest::SubscribeBalance => {
                let stream = client.subscribe_balance_changes().await;
                let _ = response_sender
                    .send(Ok(RpcResponse::SubscribeBalance(stream)))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            RpcRequest::EcashSend(amount) => {
                const TRY_CANCEL_AFTER: Duration = Duration::from_secs(60 * 60 * 24 * 3); // 3 days

                let response = client
                    .get_first_module::<MintClientModule>()
                    .spend_notes(amount, TRY_CANCEL_AFTER, false, ())
                    .await
                    .map(|(_, notes)| RpcResponse::EcashSend(notes));

                let _ = response_sender
                    .send(response)
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            RpcRequest::EcashReceive(notes) => {
                async fn receive_inner(
                    client: &Client,
                    notes: &str,
                ) -> anyhow::Result<RpcResponse> {
                    let notes = notes.trim();
                    info!("Receiving notes: \"{notes}\"");
                    let notes: OOBNotes = notes.parse()?;
                    let amount = notes.total_amount();
                    client
                        .get_first_module::<MintClientModule>()
                        .reissue_external_notes(notes, ())
                        .await?;
                    Ok(RpcResponse::EcashReceive(amount))
                }
                let _ = response_sender
                    .send(receive_inner(client, &notes).await)
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            RpcRequest::LnSend(invoice) => {
                let invoice = match Bolt11Invoice::from_str(&invoice) {
                    Ok(invoice) => invoice,
                    Err(e) => {
                        let _ = response_sender
                            .send(Err(anyhow::anyhow!("Invalid invoice: {e:?}")))
                            .map_err(|_| warn!("RPC receiver dropped before response was sent"));
                        continue;
                    }
                };

                let gateway = client
                    .get_first_module::<LightningClientModule>()
                    .list_gateways()
                    .await
                    .first()
                    .map(|gw| gw.info.clone());
                let _ = response_sender
                    .send(
                        client
                            .get_first_module::<LightningClientModule>()
                            .pay_bolt11_invoice(gateway, invoice, ())
                            .await
                            .map(|_| RpcResponse::LnSend),
                    )
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            RpcRequest::LnReceive {
                amount,
                description,
            } => {
                let gateway = client
                    .get_first_module::<LightningClientModule>()
                    .list_gateways()
                    .await
                    .first()
                    .map(|gw| gw.info.clone());
                let (operation_id, invoice, _) = match client
                    .get_first_module::<LightningClientModule>()
                    .create_bolt11_invoice(
                        amount,
                        Bolt11InvoiceDescription::Direct(
                            &lightning_invoice::Description::new(description)
                                .expect("FIXME: handle invalid descriptions"),
                        ),
                        None,
                        (),
                        gateway,
                    )
                    .await
                {
                    Ok(res) => res,
                    Err(e) => {
                        let _ = response_sender
                            .send(Err(e))
                            .map_err(|_| warn!("RPC receiver dropped before response was sent"));
                        continue;
                    }
                };

                let (await_paid_sender, await_paid_receiver) = watch::channel(false);
                let mut subscription = client
                    .get_first_module::<LightningClientModule>()
                    .subscribe_ln_receive(operation_id)
                    .await
                    .expect("subscribing to a just created operation can't fail")
                    .into_stream();
                spawn("waiting for invoice being paid", async move {
                    while let Some(state) = subscription.next().await {
                        if state == fedimint_ln_client::LnReceiveState::Funded {
                            let _ = await_paid_sender.send(true);
                        }
                    }
                });

                let _ = response_sender
                    .send(Ok(RpcResponse::LnReceive {
                        invoice: invoice.to_string(),
                        await_paid: await_paid_receiver,
                    }))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            RpcRequest::ListTransactions => {
                let transactions = client
                    .operation_log()
                    .list_operations(100, None)
                    .await
                    .into_iter()
                    .map(|(key, op_log)| {
                        let (amount_msat, description) = match op_log.operation_module_kind() {
                            "mint" => {
                                let meta = op_log.meta::<MintOperationMeta>();
                                match meta.variant {
                                    MintOperationMetaVariant::Reissuance { .. } => {
                                        (meta.amount.msats as i64, None)
                                    }
                                    MintOperationMetaVariant::SpendOOB { .. } => {
                                        (-(meta.amount.msats as i64), None)
                                    }
                                }
                            }
                            "ln" => match op_log.meta::<LightningOperationMeta>().variant {
                                LightningOperationMetaVariant::Receive { invoice, .. } => {
                                    let amount = invoice
                                        .amount_milli_satoshis()
                                        .expect("We don't create 0 amount invoices")
                                        as i64;
                                    let description = match invoice.description() {
                                        Bolt11InvoiceDescription::Direct(description) => {
                                            Some(description.to_string())
                                        }
                                        Bolt11InvoiceDescription::Hash(_) => None,
                                    };

                                    (amount, description)
                                }
                                LightningOperationMetaVariant::Pay(LightningOperationMetaPay {
                                    invoice,
                                    ..
                                }) => {
                                    // TODO: add fee
                                    let amount = -(invoice
                                        .amount_milli_satoshis()
                                        .expect("Can't pay 0 amount invoices")
                                        as i64);
                                    let description = match invoice.description() {
                                        Bolt11InvoiceDescription::Direct(description) => {
                                            Some(description.to_string())
                                        }
                                        Bolt11InvoiceDescription::Hash(_) => None,
                                    };

                                    (amount, description)
                                }
                                LightningOperationMetaVariant::Claim { .. } => {
                                    panic!("We'll never generate such operations")
                                }
                            },
                            _ => panic!("Unsupported module"),
                        };

                        Transaction {
                            timestamp: key.creation_time,
                            operation_id: key.operation_id,
                            operation_kind: op_log.operation_module_kind().to_owned(),
                            amount_msat,
                            description,
                        }
                    })
                    .collect::<Vec<_>>();
                let _ = response_sender
                    .send(Ok(RpcResponse::ListTransactions(transactions)))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            req => {
                let _ = response_sender
                    .send(Err(anyhow::anyhow!("Invalid request: {req:?}")))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
        }
    }

    info!("Client RPC handler shutting down");
}

#[derive(Clone)]
pub struct ClientRpc {
    sender: mpsc::Sender<RpcCall>,
}

impl ClientRpc {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(10);
        wasm_bindgen_futures::spawn_local(run_client(receiver));
        Self { sender }
    }

    pub async fn join(&self, invite: String) -> anyhow::Result<()> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((RpcRequest::Join(invite), response_sender))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped")?;
        match response {
            RpcResponse::Join => Ok(()),
            _ => Err(anyhow::anyhow!("Invalid response")),
        }
    }

    pub async fn get_name(&self) -> anyhow::Result<String, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((RpcRequest::GetName, response_sender))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped")?;
        match response {
            RpcResponse::GetName(name) => Ok(name),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn subscribe_balance(&self) -> anyhow::Result<BoxStream<'static, Amount>> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((RpcRequest::SubscribeBalance, response_sender))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped")?;
        match response {
            RpcResponse::SubscribeBalance(stream) => Ok(stream),
            _ => Err(anyhow::anyhow!("Invalid response")),
        }
    }

    pub async fn ecash_send(&self, amount: Amount) -> anyhow::Result<OOBNotes, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((RpcRequest::EcashSend(amount), response_sender))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped")?;
        match response {
            RpcResponse::EcashSend(notes) => Ok(notes),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn ecash_receive(&self, invoice: String) -> anyhow::Result<Amount, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((RpcRequest::EcashReceive(invoice), response_sender))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped")?;
        match response {
            RpcResponse::EcashReceive(amount) => Ok(amount),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn ln_send(&self, invoice: String) -> anyhow::Result<(), RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((RpcRequest::LnSend(invoice), response_sender))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped")?;
        match response {
            RpcResponse::LnSend => Ok(()),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn ln_receive(
        &self,
        amount_msat: u64,
        description: String,
    ) -> anyhow::Result<(String, watch::Receiver<bool>), RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::LnReceive {
                    amount: Amount::from_msats(amount_msat),
                    description,
                },
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped")?;
        match response {
            RpcResponse::LnReceive {
                invoice,
                await_paid,
            } => Ok((invoice, await_paid)),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn list_transactions(&self) -> anyhow::Result<Vec<Transaction>, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((RpcRequest::ListTransactions, response_sender))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped")?;
        match response {
            RpcResponse::ListTransactions(transactions) => Ok(transactions),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn list_wallets(&self) -> Result<Vec<String>, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((RpcRequest::ListWallets, response_sender))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped")?;
        match response {
            RpcResponse::ListWallets(wallets) => Ok(wallets),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    /// Opens a wallet and returns whether it is initialized already. If false
    /// is returned an invite code has to be provided.
    pub async fn select_wallet(&self, name: String) -> Result<bool, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((RpcRequest::SelectWallet(name), response_sender))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped")?;
        match response {
            RpcResponse::SelectWallet { initialized } => Ok(initialized),
            _ => Err(RpcError::InvalidResponse),
        }
    }
}
