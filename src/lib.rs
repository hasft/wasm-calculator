use yew::{html, Component, ComponentLink, Renderable, ShouldRender, Html};
use yew::services::ConsoleService;

pub struct Model {
    console: ConsoleService,
    value: i32,
    input: i32
}

pub enum Msg {
    Add,
    Substract,
    Multiply,
    Divide,
    Clear,
    Calculate,
    Update(String)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            input: 0,
            value: 0
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {                
                self.value = self.value + 1;
                self.console.log(&self.value.to_string());
            }
            Msg::Substract => {

            }
            Msg::Multiply => {

            }
            Msg::Divide => {

            }
            Msg::Clear => {

            }
            Msg::Calculate => {

            }
            Msg::Update(val) => {
                println!("Input: {}", val);
                self.value = val.parse::<i32>().unwrap();
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <input class="toggle-all", type="text", value=self.value, oninput=|e| Msg::Update(e.value), />
                <button onclick=|_| Msg::Add,>{ "+" }</button>
            </div>
        }
    }
}
