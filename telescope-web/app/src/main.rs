use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::console;
use wasm_bindgen_futures::spawn_local;

mod api;

#[derive(Debug)]
pub struct Model;
impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        
        spawn_local(async {
            let repos: String = match api::get_repos().await {
                Ok(val) => val,
                Err(e) => format!("{}", e)
            };

            console::log_1(&JsValue::from(repos));
        });

        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main>
                <h1>{ "Yew ❤️ Atmo!" }</h1>
            </main>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
