use dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
    let text = cx.use_hook(|| vec![String::from("abc=def")]);

    let first = text.get_mut(0).unwrap();
    cx.render(rsx! {
        div { child_1 { text: first } }
    })
}

#[derive(Props)]
struct C1Props<'a> {
    text: &'a mut String,
}

fn child_1<'a>(cx: Scope<'a, C1Props<'a>>) -> Element {
    let (left, right) = cx.props.text.split_once("=").unwrap();
    cx.render(rsx! {
        div {
            child_2 { text: left }
            child_2 { text: right }
        }
    })
}
#[derive(Props)]
struct C2Props<'a> {
    text: &'a str,
}
fn child_2<'a>(cx: Scope<'a, C2Props<'a>>) -> Element {
    cx.render(rsx! { child_3 { text: cx.props.text } })
}

#[derive(Props)]
struct C3Props<'a> {
    text: &'a str,
}
fn child_3<'a>(cx: Scope<'a, C3Props<'a>>) -> Element {
    cx.render(rsx! { div { "{cx.props.text}" } })
}
