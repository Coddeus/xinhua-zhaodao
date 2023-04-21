use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct SearchArgs<'a> {
    cmp: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let search_input = use_node_ref();
    // Searched component
    let cmp = use_state(|| String::new());
    // Search result (derivatives)
    let derivs = use_state(|| String::new());
    {
        let derivs = derivs.clone();
        let cmp = cmp.clone();
        let cmp2 = cmp.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if cmp.is_empty() {
                        return;
                    }

                    let args = to_value(&SearchArgs { cmp: &*cmp }).unwrap();
                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    let new_derivs = invoke("derivatives", args).await.as_string().unwrap();
                    derivs.set(new_derivs);
                });

                || {}
            },
            cmp2,
        );
    }

    let greet = {
        let cmp = cmp.clone();
        let search_input = search_input.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            cmp.set(
                search_input
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <main class="container">
            <form class="row" onsubmit={greet}>
                <input id="greet-input" ref={search_input} placeholder="Enter a character…" />
                <button type="submit">{"Look up"}</button>
            </form>

            <p><b>{ &*derivs }</b></p>
        </main>
    }
}