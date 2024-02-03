mod component;

use custom_elements::{CustomElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};
use yew::prelude::*;

struct ComponentWrapper {
    app: Option<AppHandle<component::App>>,
}

impl ComponentWrapper {
    fn new() -> Self {
        Self { app: None }
    }
}

impl CustomElement for ComponentWrapper {
    fn inject_children(&mut self, this: &HtmlElement) {
        let app = yew::Renderer::<component::App>::with_root(this.clone().unchecked_into()).render();
        self.app = Some(app);
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["value"]
    }
}

impl Default for ComponentWrapper {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    ComponentWrapper::define("example-page");
}
