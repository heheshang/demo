use dioxus::prelude::*;
use dioxus_desktop::use_window;
use futures_util::stream::StreamExt;

fn app(cx: Scope) -> Element {
    let window = use_window(cx);
    let emails_sent = use_ref(cx, Vec::new);

    let tx = use_coroutine(cx, |mut rx: UnboundedReceiver<String>| {
        to_owned![emails_sent];
        async move {
            while let Some(message) = rx.next().await {
                emails_sent.write().push(message);
            }
        }
    });
    cx.render(rsx! {
        div { h1 { "This is your email inbox" } }
    })
}

struct ComposeProps {
    app_tx: Coroutine<String>,
}

fn compose(cx: Scope<ComposeProps>) -> Element {
    let user_input = use_state(cx, String::new);
    let window = use_window(cx);
    cx.render(rsx! {
        div {
            h1 { "Compose a new email" }
            button {
                onclick: move |_| {
                    let email = user_input.get().clone();
                    cx.props.app_tx.send(email);
                    window.close();
                },
                "Click to send"
            }
            input {
                value: "{user_input}",
                oninput: move |evt| {
                    user_input.set(evt.value.clone());
                }
            }
        }
    })
}
