//! Example: README.md showcase
//!
//! The example from the README.md.
//!

use dioxus::prelude::*;

use crate::{use_init_signal_rt, use_signal};

pub fn app(cx: Scope) -> Element {
    use_init_signal_rt(cx);

    let mut count = use_signal(cx, || 0);

    use_future!(cx, || async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            count += 1;
            log::info!("current: {count}");
        }
    });

    cx.render(rsx! { div { "High-Five counter: {count}" } })
}
