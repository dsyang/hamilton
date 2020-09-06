use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct HamiltonApp {}

impl Component for HamiltonApp {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {<span>{"Hello World!"}</span>}
    }
}


#[wasm_bindgen(start)]
pub fn run_app() {
    App::<HamiltonApp>::new().mount_to_body();
}

