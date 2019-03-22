#![recursion_limit="256"]  
extern crate meval;
//extern crate regex;

use yew::{html, Component, ComponentLink, Renderable, ShouldRender, Html};
use yew::services::ConsoleService;
//use regex::{Regex, RegexSet};

pub struct Model {
    console: ConsoleService,
    input: String,
    result: String,
}

pub enum Msg {
    InsertNumber(String),
    InsertOperator(String),
    RemoveNumber,
    Calculate,
    PositiveNegative,
    Percentage,
    SetFloat,
    Clear,
    InsertNill
}


fn last_is_operator(current_val: &String) -> bool {
    let _last = current_val.chars().nth(current_val.len() - 1).unwrap();

    if current_val.is_empty() {
        return false
    } else {
        match _last {
            '+' | '-' | '*' | '/' => false,
            _ => true
        }
    }
}

fn last_is_number(current_val: &String) -> bool {
    let _last = current_val.chars().nth(current_val.len() - 1).unwrap();

    if current_val.is_empty() {
        return false
    } else {
        match _last {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
            _ => false
        }
    }
}

fn valid_for_negative(current_val: &String) -> bool {
    let _first = current_val.chars().nth(0).unwrap();
    match _first {
        '-' => false,
        _ => true
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            input: String::new(),
            result: String::from("0"),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InsertNumber(_val) => {
                self.input.push_str(&_val[..]);
            }
            Msg::InsertOperator(_val) => {
                if !last_is_operator(&self.input) {
                    self.console.log("true");
                    return false;
                } else {
                    self.input.push_str(&_val[..]);
                }
            }
            Msg::RemoveNumber => {
                let len = self.input.len() - 1;
                self.input.remove(len).to_string();
            }
            Msg::Calculate => {
                if self.input.is_empty() {
                    return false
                } else {
                    let r:f64 = meval::eval_str(&self.input).unwrap();
                    self.result = r.to_string();
                    self.input.clear();
                }
            }
            Msg::Clear => {
                self.result = String::from("0");
                self.input = String::from("");
            }
            Msg::PositiveNegative => {
                if valid_for_negative(&self.input) {
                    self.input = "-".to_owned() + &self.input;
                } else {
                    return false
                }
            }
            Msg::Percentage => {

            }
            Msg::SetFloat => {

            }
            Msg::InsertNill => {
                if last_is_number(&self.input) {
                    self.input.push_str("0");
                } else {
                    return false
                }
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="container",>
                <div class="app",>
                <div class="calculator",>
                <div class="display",>
                <div class="display__current",>{&self.input}</div>
                <div class="display__result",>{&self.result}</div>
                </div>
            // end of display
                <div class="controls",>
                <a class="controls__item controls__item--darker", onclick=|_| Msg::Clear,>{"Ac"}</a>
                <a class="controls__item controls__item--darker", onclick=|_| Msg::PositiveNegative,>{"+/-"}</a>
                <a class="controls__item controls__item--darker", onclick=|_| Msg::Percentage,>{"%"}</a>
                <a class="controls__item controls__item--orangeLight", onclick=|_| Msg::InsertOperator("/".to_string()),>{ "/" }</a>
                <a class="controls__item", onclick=|_| Msg::InsertNill,>{"0"}</a>
                <a class="controls__item", onclick=|_| Msg::InsertNumber("1".to_string()),>{"1"}</a>
                <a class="controls__item", onclick=|_| Msg::InsertNumber("2".to_string()),>{"2"}</a>
                <a class="controls__item controls__item--orangeLight", onclick=|_| Msg::InsertOperator("*".to_string()),>{ "x" }</a>
                <a class="controls__item", onclick=|_| Msg::InsertNumber("3".to_string()),>{"3"}</a>
                <a class="controls__item", onclick=|_| Msg::InsertNumber("4".to_string()),>{"4"}</a>
                <a class="controls__item", onclick=|_| Msg::InsertNumber("5".to_string()),>{"5"}</a>
                <a class="controls__item controls__item--orangeLight", onclick=|_| Msg::InsertOperator("+".to_string()),>{ "+" }</a>
                <a class="controls__item", onclick=|_| Msg::InsertNumber("6".to_string()),>{"6"}</a>
                <a class="controls__item", onclick=|_| Msg::InsertNumber("7".to_string()),>{"7"}</a>
                <a class="controls__item", onclick=|_| Msg::InsertNumber("8".to_string()),>{"8"}</a>
                <a class="controls__item controls__item--orangeLight", onclick=|_| Msg::InsertOperator("-".to_string()),>{ "-" }</a>
                <a class="controls__item", onclick=|_| Msg::InsertNumber("9".to_string()),>{"9"}</a>
                <a class="controls__item", onclick=|_| Msg::InsertNumber(".".to_string()),>{"."}</a>
                <a class="controls__item", onclick=|_| Msg::RemoveNumber,>{"Del"}</a>
                <a class="controls__item controls__item--orangeMedium", onclick=|_| Msg::Calculate,>{ "=" }</a>
            </div>
            // end of controls
                </div>
                </div>
                </div>
        }
    }
}
