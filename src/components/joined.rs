use leptos::*;

use crate::context::ClientContext;

use crate::components::{Balance, Receive, ReceiveLn, Send, TxList};
use crate::utils::empty_view;

#[derive(Clone, PartialEq)]
enum Tab {
    TxList,
    Send,
    ReceiveLn,
    Receive,
}

//
// Joined component
// First view whenever an user joined a Federation
//
#[component]
pub fn Joined() -> impl IntoView {
    let ClientContext { client, .. } = expect_context::<ClientContext>();

    // get name of the federation
    let name_resource = create_resource(
        || (),
        move |_| async move { client.get_value().get_name().await },
    );

    let federation_label = move || {
        name_resource
            .read()
            .map(|value| match value {
                Err(error) => format!("Failed to get federation name {error:?}"),
                Ok(value) => value,
            })
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".into())
    };

    let (tab, set_tab) = create_signal(Tab::Receive);

    view! {
      <h1 class="font-heading text-gray-900 text-4xl font-semibold">{federation_label}</h1>
      <Balance class="my-12" />
      <ul
        class="my-12 w-full flex flex-row"
        >
        <li class="w-1/4">
        <button
          on:click=move |_| {
            set_tab.set(Tab::TxList);
          }
          class={move || format!("my-2 block w-full text-center
          border-b-2 
          py-4
          ease
          font-body font-semibold  
          text-xl leading-tight hover:text-blue-500 {active}", 
          active = if tab.get() == Tab::TxList  {"text-blue-400 border-blue-400"} else {"text-gray-400 border-gray-200 hover:border-gray-700"} )}
          >
            Transactions
        </button>
      </li>
        <li class="w-1/4">
        <button
          on:click=move |_| {
            set_tab.set(Tab::Receive);
          }
          class={move || format!("my-2 block w-full text-center
          border-b-2 
          py-4
          ease
          font-body font-semibold  
          text-xl leading-tight hover:text-blue-500 {active}", 
          active = if tab.get() == Tab::Receive  {"text-blue-400 border-blue-400"} else {"text-gray-400 border-gray-200 hover:border-gray-700"} )}

          >Redeem
        </button>
      </li>
        <li class="w-1/4">
          <button
            on:click=move |_| {
              set_tab.set(Tab::Send);
            }
            class={move || format!("my-2 block w-full text-center
            border-b-2 
            py-4
            ease
            font-body font-semibold  
            text-xl leading-tight hover:text-blue-500 {active}", 
            active = if tab.get() == Tab::Send {"text-blue-400 border-blue-400"} else {"text-gray-400 border-gray-200 hover:border-gray-700"} )}
            >LN Send
          </button>
        </li>
        <li class="w-1/4">
          <button
            on:click=move |_| {
              set_tab.set(Tab::ReceiveLn);
            }
            class={move || format!("my-2 block w-full text-center
            border-b-2 
            py-4
            ease
            font-body font-semibold  
            text-xl leading-tight hover:text-blue-500 {active}", 
            active = if tab.get() == Tab::ReceiveLn  {"text-blue-400 border-blue-400"} else {"text-gray-400 border-gray-200 hover:border-gray-700"} )}
          >LN Receive</button>
        </li>
      </ul>

      <Show
          when=move || tab.get() == Tab::Send
          fallback=|_| empty_view()
          >
          <Send />
      </Show>
      <Show
          when=move || tab.get() == Tab::Receive
          fallback=|_| empty_view()
          >
          <Receive />
      </Show>
      <Show
          when=move || tab.get() == Tab::ReceiveLn
          fallback=|_| empty_view()
          >
          <ReceiveLn />
      </Show>
      <Show
          when=move || tab.get() == Tab::TxList
          fallback=|_| empty_view()
          >
          <TxList update_signal=move || {tab.get();} />
      </Show>

    }
}
