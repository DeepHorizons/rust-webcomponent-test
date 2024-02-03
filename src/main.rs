use yew::prelude::*;
use wasm_bindgen::prelude::*;


#[wasm_bindgen(module = "/example-page/pkg/example_page.js")]
extern "C" {
    #[wasm_bindgen(js_name = default)]
    async fn ep_init();

    #[wasm_bindgen(js_name = run)]
    fn ep_run();
}


#[wasm_bindgen(module = "/example-page/pkg/snippets/custom-elements-419327fa2779072f/src/make_custom_element.js")]
extern "C" {
    fn make_custom_element();
}

// TODO how do I include the .wasm file


#[function_component]
fn App() -> Html {
    html! {
        <>
            <h1>{"Top level"}</h1>
            <welcome-page></welcome-page>
            <h1>{"End"}</h1>
        </>
    }
}

fn main() {
    wasm_bindgen_futures::spawn_local(ep_init());
    ep_run();
    yew::Renderer::<App>::new().render();
}
