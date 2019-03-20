extern crate meval;

use yew::{html, Component, ComponentLink, Renderable, ShouldRender, Html};
use yew::services::ConsoleService;

pub struct Model {
    console: ConsoleService,
    input: String,
    calculated: bool
}

pub enum Msg {
    Update(String),
    Calculate,
    Clear
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            input: String::new(),
            calculated: false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(_val) => {
                if self.calculated == true {
                    self.input = _val;
                    self.calculated = false;
                } else {
                    self.input.push_str(&_val);
                    self.console.log("update");
                }
            }
            Msg::Calculate => {
                let r:f64 = meval::eval_str(&self.input).unwrap();
                self.input = r.to_string();
                self.calculated = true;
            }
            Msg::Clear => {
                self.input = "".to_string();
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <input class="toggle-all", type="text", value=&self.input, oninput=|e| Msg::Update(e.value), />
                <button onclick=|_| Msg::Clear,>{"c"}</button>
                <button onclick=|_| Msg::Calculate,>{ "=" }</button>
                <button onclick=|_| Msg::Update(String::from("+")),>{ "+" }</button>
                <button onclick=|_| Msg::Update(String::from("-")),>{ "-" }</button>
                <button onclick=|_| Msg::Update(String::from("*")),>{ "x" }</button>
                <button onclick=|_| Msg::Update(String::from("/")),>{ "/" }</button>
                <ul> {
                    for (0..9).map(|x| html! {<li><button onclick=|_| Msg::Update(x.to_string()),>{x}</button></li>})
                } </ul>
             </div>
        }
    }
}
