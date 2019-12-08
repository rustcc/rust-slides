use slides::SLIDE_MARKDOWN;
use std::time::Duration;
use yew::prelude::*;
use yew::services::{Task, TimeoutService};

const FRAME_RATE_MILLI: u64 = 25;
const OPACITY_STEP: f32 = 0.05;

#[derive(Copy, Clone)]
pub enum TransitionType {
    Show,
    Fade,
}

#[derive(Clone)]
pub enum Msg {
    GotKeyPress(KeyDownEvent),
    Transition(TransitionType, usize),
    GoLeft,
    GoRight,
}

pub struct Model {
    pub slides: Vec<&'static str>,
    pub slide_idx: usize,
    pub opacity: f32,
    // need to keep ref to task returned by timeout service otherwise it gets dropped before completing
    pub handler: Option<Box<Task>>,

    timeout: TimeoutService,
    link: ComponentLink<Model>,
}

impl Model {
    pub fn new(link: ComponentLink<Self>) -> Model {
        let timeout = TimeoutService::new();
        let mut model = Model {
            link,
            timeout,
            opacity: 0.0,
            slides: SLIDE_MARKDOWN
                .split("---\n")
                .map(|s| s.trim())
                .filter(|s| s != &"")
                .collect(),
            slide_idx: 0,
            handler: None,
        };
        model.transition(TransitionType::Show, 0);
        model
    }

    pub fn transition(&mut self, t_type: TransitionType, next_slide: usize) {
        if self.opacity > 1.0 {
            self.opacity = 1.0;
        }
        if self.opacity < 0.0 {
            self.opacity = 0.0;
        }
        match t_type {
            TransitionType::Show => {
                self.opacity += OPACITY_STEP;
            }
            TransitionType::Fade => {
                self.opacity -= OPACITY_STEP;
            }
        }

        let send_msg = self
            .link
            .send_back(move |_| Msg::Transition(t_type, next_slide));
        let handle = self
            .timeout
            .spawn(Duration::from_millis(FRAME_RATE_MILLI), send_msg);
        self.handler = Some(Box::new(handle));
    }

    pub fn handle_transition(&mut self, t_type: TransitionType, next_slide: usize) -> bool {
        let frame = self.opacity;
        if frame >= 1.0 {
            self.opacity = frame;
            self.handler = None;
        } else if frame <= 0.0 {
            self.slide_idx = next_slide;
            self.transition(TransitionType::Show, next_slide);
        } else {
            self.transition(t_type, next_slide);
        }
        true
    }

    pub fn go_left(&mut self) -> ShouldRender {
        if self.slide_idx > 0 {
            let next_slide = self.slide_idx - 1;
            if let Some(mut task) = self.handler.take() {
                task.cancel();
            }
            self.transition(TransitionType::Fade, next_slide);
            true
        } else {
            false
        }
    }

    pub fn go_right(&mut self) -> ShouldRender {
        if self.slide_idx < self.slides.len() - 1 {
            let next_slide = self.slide_idx + 1;
            if let Some(mut task) = self.handler.take() {
                task.cancel();
            }

            self.transition(TransitionType::Fade, next_slide);
            true
        } else {
            false
        }
    }
}
