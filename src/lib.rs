use wasm_bindgen::prelude::*;
use yew::{prelude::*, services::ConsoleService};

mod artnet;

struct Model {
    link: ComponentLink<Self>
}

enum Msg {
    Start,
    Stop
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Start => artnet::ArtNetReceiver::start().unwrap(),
            Msg::Stop => ConsoleService::info(format!("Stop").as_ref())
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::Start)>{ "Start" }</button>
                <button onclick=self.link.callback(|_| Msg::Stop)>{ "Stop" }</button>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}