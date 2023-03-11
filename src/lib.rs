use yew::prelude::*;
use yew::{html, Component, Context, Html};
use wasm_bindgen::prelude::*;

pub struct App {

}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {  }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {


        html! {
            <header>
                <div class="inner">
                    <div class="logo">
                        <img src="https://avatars.githubusercontent.com/u/125027502?v=4" alt="Lattexshz" />
                    </div>
                    <nav>
                        <ul>
                            <li><a href="/posts">{"Posts"}</a></li>
                        </ul>
                    </nav>
                </div>
            </header>
        }
    }
}

#[wasm_bindgen(start)]
fn main() {
    yew::Renderer::<App>::new().render();
}

