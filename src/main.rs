use demo::{all_event, borrowed, calculator, callback, clock, todomvc};
// use dioxus::prelude::*;
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(home);
    // dioxus_web::launch(all_event::app);
}

use dioxus::prelude::*;
use dioxus_router::{Link, Route, Router};
pub fn home(cx: Scope) -> Element {
    cx.render(rsx! {

        Router { 

            ul {
                Link { to: "/todomvc", li { "Todo Mvc!" } }
                Link { to: "/all_events", li { "All Events" } }
                Link { to: "/borrowed", li { "Borrowed" } }
                Link { to: "/calculator", li { "calculator" } }
                Link { to: "/callback", li { "callback" } }
                Link { to: "/clock", li { "clock" } }
            }

            Route { to: "/todomvc", todomvc::app {} }
            Route { to: "/all_events", all_event::app {} }
            Route { to: "/borrowed", borrowed::app {} }
            Route { to: "/calculator", calculator::app {} }
            Route { to: "/callback", callback::app {} }
            Route { to: "/clock", clock::app {} }
        }
    })
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        print!("test")
    }
}
