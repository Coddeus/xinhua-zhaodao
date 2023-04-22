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
    let deriv_zi = use_state(|| String::new());
    let deriv_ci = use_state(|| String::new());
    {
        let deriv_zi = deriv_zi.clone();
        let deriv_ci = deriv_ci.clone();
        let cmp = cmp.clone();
        let cmp2 = cmp.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if cmp.is_empty() {
                        deriv_zi.set("".to_owned());
                        deriv_ci.set("".to_owned());
                        return
                    }

                    let args_zi = to_value(&SearchArgs { cmp: &*cmp }).unwrap();
                    let args_ci = to_value(&SearchArgs { cmp: &*cmp }).unwrap();
                    let new_deriv_zi = invoke("derivative_zi", args_zi).await.as_string().unwrap();
                    let new_deriv_ci = invoke("derivative_ci", args_ci).await.as_string().unwrap();
                    deriv_zi.set(new_deriv_zi);
                    deriv_ci.set(new_deriv_ci);
                });

                || {}
            },
            cmp2,
        );
    }
    let search = {
        let cmp = cmp.clone();
        let search_input = search_input.clone();
        Callback::from(move |e: InputEvent| {
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
            <div><input id="search" ref={search_input} style="width: 20%; text-align: center;" placeholder="找 … 的转字和转词" oninput={search}/></div>
            <br/>
            <p>{"字："}</p>
            <br/>
            <p style="margin: 0 15% 0 15%;">{ &*deriv_zi }</p>
            <br/>
            <p>{"词："}</p>
            <br/>
            <p style="margin: 0 15% 0 15%;">{ &*deriv_ci }</p>
        </main>
    }
}