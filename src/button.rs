use dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            button { onclick: |_| async move {
                    log::info!("hello, click");
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    log::info!("goodbye,click");
                },
                "hello click"
            }
        }
    })
}
