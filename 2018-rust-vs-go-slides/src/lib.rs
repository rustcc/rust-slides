#[macro_use]
extern crate yew;

extern crate pulldown_cmark;

#[macro_use]
extern crate stdweb;

mod markdown;
pub mod model;
mod slides;

use markdown::render_markdown;
use model::{Model, Msg};
use yew::prelude::*;

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model::new(link)
    }

    fn update(&mut self, cmd: Self::Message) -> ShouldRender {
        match cmd {
            Msg::Transition(t_type, next_slide) => self.handle_transition(t_type, next_slide),
            Msg::GotKeyPress(event) => match event.key().as_str() {
                "ArrowLeft" | "Backspace" => self.go_left(),
                "ArrowRight" | "Enter" => self.go_right(),
                _ => false,
            },
            Msg::GoLeft => self.go_left(),
            Msg::GoRight => self.go_right(),
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div id="container", tabindex="-1", onkeydown=|e| Msg::GotKeyPress(e), >
                <div id="goLeft", onclick=|_| Msg::GoLeft, />
                <div id="goRight", onclick=|_| Msg::GoRight, />
                <div id="pageCount", >
                <span> {format!("{}/{}", self.slide_idx + 1, self.slides.len())}</span>
                </div>
                <div id="content", >
                <div style={format!("opacity: {};", self.opacity)}, >
            {render_markdown(self.slides[self.slide_idx])} </div>
                </div>
                </div>
                </div>
        }
    }
}
