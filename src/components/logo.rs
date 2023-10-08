use leptos::*;

#[component]
pub fn Logo(cx: Scope, #[prop(optional, into)] class: String) -> impl IntoView {
    // Name needs to be defined! https://discord.com/channels/990354215060795454/1140921151707684984/1140921156149465130
    view! { cx,
      <svg class=class width="205" height="30" version="1.1"  viewBox="0 0 54.239583 7.9375" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
        <defs>
          <radialGradient id="a" cx="0" cy="0" r="1" gradientTransform="matrix(-8.438 0 0 -8.438 4.0578 4.2355)" gradientUnits="userSpaceOnUse">
          <stop stop-color="#4AD6FF" offset=".50796"/>
          <stop stop-color="#181884" offset="1"/>
          </radialGradient>
          <radialGradient id="b" cx="0" cy="0" r="1" gradientTransform="matrix(-7.7934 0 0 -7.7934 -.67225 8.0704)" gradientUnits="userSpaceOnUse">
          <stop stop-color="#4AD6FF" offset=".50796"/>
          <stop stop-color="#181884" offset="1"/>
          </radialGradient>
        </defs>
        <path
        d="M 10.219147,7.7928796 9.572468,2.2125521 h 1.340586 l 0.326973,3.9236678 h 0.05086 l 1.65666,-3.9236678 h 1.326054 l 0.348771,3.9018696 h 0.05449 l 1.62033,-3.9018696 h 1.344219 L 15.13099,7.7928796 H 13.76134 L 13.339916,4.0217989 h -0.07629 L 11.59243,7.7928796 Z m 9.716858,0.1089908 q -0.839229,0 -1.387816,-0.3487705 Q 17.999603,7.2006964 17.777988,6.5576508 17.560006,5.9109723 17.705328,5.0354131 17.847016,4.174386 18.28298,3.5240744 q 0.435963,-0.6539446 1.089907,-1.0172472 0.657578,-0.3669356 1.453211,-0.3669356 0.515889,0 0.948219,0.1671192 0.43233,0.1634861 0.730238,0.5086236 0.297908,0.3451374 0.410532,0.8791922 0.112624,0.5304217 -0.0109,1.2642929 l -0.06176,0.4032659 h -4.577613 l 0.141688,-0.8864583 h 3.31332 q 0.06539,-0.3778347 -0.0436,-0.6721097 -0.108991,-0.2979081 -0.363303,-0.4686603 -0.254312,-0.1707523 -0.63578,-0.1707523 -0.388733,0 -0.726605,0.1998165 -0.334238,0.1961833 -0.563119,0.5086236 -0.22888,0.3124402 -0.290642,0.6575776 l -0.148954,0.850128 q -0.07993,0.5231557 0.03633,0.8537611 0.116257,0.3306053 0.406899,0.4868254 0.290642,0.1562201 0.733872,0.1562201 0.290642,0 0.537687,-0.079927 0.247046,-0.079927 0.439596,-0.2397797 0.196184,-0.1634861 0.326973,-0.3996328 l 1.202531,0.138055 Q 22.431891,6.782898 22.04679,7.1462006 21.661689,7.5058702 21.124002,7.7056866 20.589947,7.90187 19.936002,7.90187 Z m 3.672973,-0.1089908 1.235228,-7.44043664 h 1.315156 L 25.7016,3.1353406 h 0.05813 q 0.130789,-0.2034494 0.352403,-0.43233 0.225248,-0.2325137 0.570385,-0.3959998 0.345138,-0.1671192 0.835596,-0.1671192 0.646679,0 1.111706,0.3306053 0.465027,0.3269724 0.664844,0.9700179 0.199816,0.6394125 0.04723,1.5694671 -0.152587,0.9191555 -0.559486,1.562201 -0.406899,0.6430456 -0.98455,0.980917 -0.577651,0.3378714 -1.235229,0.3378714 -0.479559,0 -0.770201,-0.1598532 Q 25.505417,7.571265 25.35283,7.3460174 25.200243,7.1171368 25.131215,6.9136874 h -0.08356 l -0.145321,0.8791922 z m 1.754751,-2.7901638 q -0.08719,0.5413209 -0.0036,0.9482198 0.08719,0.4068988 0.341504,0.6357795 0.257945,0.2252475 0.679376,0.2252475 0.43233,0 0.762936,-0.2325136 0.334238,-0.2361467 0.552219,-0.6430455 0.217982,-0.410532 0.305175,-0.9336877 0.08356,-0.5195226 0,-0.9227885 -0.07993,-0.4032658 -0.337872,-0.6321464 -0.254312,-0.2288807 -0.690275,-0.2288807 -0.425064,0 -0.755669,0.2216146 -0.330605,0.2216146 -0.548587,0.6212474 -0.217981,0.3996328 -0.305174,0.9409536 z m 4.635727,2.7901638 0.930054,-5.5803275 h 1.315156 l -0.930055,5.5803275 z m 1.780182,-6.3795931 q -0.316073,0 -0.523155,-0.2107155 -0.207083,-0.21071552 -0.181652,-0.5049906 0.02543,-0.29427508 0.265211,-0.50499057 0.243413,-0.21071549 0.555853,-0.21071549 0.316073,0 0.519523,0.21071549 0.207082,0.21071549 0.181651,0.50499057 -0.0218,0.29427508 -0.265211,0.5049906 -0.23978,0.2107155 -0.55222,0.2107155 z m 0.890082,6.3795931 0.930055,-5.5803275 h 1.26066 l -0.159853,0.9482197 h 0.06903 q 0.254312,-0.4795594 0.701174,-0.7484033 0.446862,-0.2724769 1.006348,-0.2724769 0.563119,0 0.908257,0.27611 0.34877,0.2724769 0.428697,0.7447702 h 0.05813 q 0.261578,-0.4686603 0.748403,-0.7447702 0.490459,-0.27611 1.100807,-0.27611 0.762936,0 1.169835,0.4868254 0.406898,0.4868255 0.250678,1.4205131 l -0.62488,3.7456495 h -1.315155 l 0.595816,-3.5422001 q 0.07993,-0.5195226 -0.159853,-0.7593023 -0.236147,-0.2434128 -0.63578,-0.2434128 -0.475926,0 -0.791999,0.2942751 -0.312441,0.2942751 -0.385101,0.7702015 L 37.240072,7.7928796 H 35.950348 L 36.55343,4.1961841 q 0.06539,-0.43233 -0.152587,-0.6902748 -0.214349,-0.2579449 -0.632147,-0.2579449 -0.283376,0 -0.541321,0.1453211 -0.257944,0.145321 -0.439596,0.4068988 -0.178018,0.2579449 -0.232513,0.5994493 l -0.566752,3.393246 z m 9.180644,0 0.930055,-5.5803275 h 1.315155 l -0.930055,5.5803275 z m 1.780183,-6.3795931 q -0.316074,0 -0.523156,-0.2107155 -0.207083,-0.21071552 -0.181651,-0.5049906 0.02543,-0.29427508 0.26521,-0.50499057 0.243413,-0.21071549 0.555853,-0.21071549 0.316074,0 0.519523,0.21071549 0.207082,0.21071549 0.181651,0.50499057 -0.0218,0.29427508 -0.265211,0.5049906 -0.239779,0.2107155 -0.552219,0.2107155 z m 2.74656,3.10987 -0.537688,3.2697231 h -1.318788 l 0.930054,-5.5803275 h 1.26066 l -0.159853,0.9482197 h 0.06903 q 0.261578,-0.4686603 0.733872,-0.7447702 0.472293,-0.27611 1.100806,-0.27611 0.570385,0 0.959119,0.2470457 0.388734,0.2434128 0.55222,0.7157061 0.163486,0.4686603 0.05086,1.1371371 L 49.421582,7.7928796 H 48.106427 L 48.66228,4.4432299 Q 48.756739,3.887377 48.520592,3.5713037 48.288078,3.2515975 47.772189,3.2515975 q -0.345138,0 -0.63578,0.152587 -0.290642,0.1489541 -0.490458,0.4323301 -0.196184,0.283376 -0.268844,0.6866419 z M 54.471483,2.2125521 54.30073,3.2297993 H 51.096402 L 51.263521,2.2125521 Z M 52.273502,0.87559866 h 1.315155 L 52.716731,6.1144217 q -0.03996,0.2652109 0.01453,0.4068989 0.05813,0.138055 0.185285,0.1889173 0.130789,0.050862 0.294275,0.050862 0.11989,0 0.225247,-0.018165 0.105358,-0.021798 0.167119,-0.032697 l 0.05813,1.0281463 q -0.11989,0.03633 -0.323339,0.079927 -0.199817,0.043596 -0.47956,0.050862 Q 52.357062,7.8800722 51.986493,7.716586 51.619557,7.5494669 51.448805,7.2043294 51.281686,6.855559 51.372512,6.3324033 Z"
        id="text2"
        inkscape:label="text_as_path"
        aria-label="webimint" />
        <path d="m4.0556 0c7.82e-4 0.0017733 0.00454-0.0015861 0 0zm0.00103 5.1676e-4c-1.0842 0.5221-2.1683 1.0442-3.2525 1.5663-0.26803 1.1737-0.53606 2.3475-0.80409 3.5212 0.75034 0.94103 1.5007 1.8821 2.251 2.8231h3.6117c0.75034-0.94103 1.5007-1.8821 2.251-2.8231-0.26803-1.1737-0.53606-2.3475-0.80409-3.5212-1.0843-0.5221-2.1687-1.0442-3.253-1.5663zm-0.0025803 1.8345c0.60806 0.29283 1.2161 0.58567 1.8242 0.8785 0.15021 0.65801 0.30041 1.316 0.45062 1.974-0.42082 0.52762-0.84164 1.0552-1.2625 1.5828h-2.0247c-0.42082-0.52762-0.84164-1.0552-1.2625-1.5828 0.15021-0.65801 0.30041-1.316 0.45062-1.974 0.60806-0.29283 1.2161-0.58567 1.8242-0.8785z" clip-rule="evenodd" fill="url(#a)" fill-rule="evenodd" stroke-width=".32717"/>
        <path d="m5.1586 4.2399c0.0060766 0.43768-0.27523 0.85979-0.67979 1.0256-0.39744 0.1731-0.89114 0.083531-1.2012-0.21989-0.31417-0.29193-0.42882-0.77287-0.28218-1.1756 0.1418-0.41741 0.55185-0.72516 0.99284-0.7417 0.42869-0.027709 0.85719 0.22076 1.0486 0.60487 0.080188 0.15604 0.12203 0.3313 0.12176 0.5067z" clip-rule="evenodd" fill="url(#b)" fill-rule="evenodd" stroke-width=".30218"/>
      </svg>
    }
}
