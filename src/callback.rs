use dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
    let login = use_callback!(cx, move |_| async move {
        let res = reqwest::get("https://dog.ceo/api/breeds/list/all")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        log::info!("{res:#?},")
    });
    cx.render(rsx! {
        div { button { onclick: login, "Click me" } }
    })
}
