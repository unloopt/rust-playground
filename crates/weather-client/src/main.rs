use axum::{
    extract::{Host, WebSocketUpgrade},
    response::Html,
    routing::get,
    Router,
};
use dioxus::prelude::*;
use tower_http::services::ServeDir;

fn app(cx: Scope) -> Element {
    let mut num = use_state(cx, || 0);

    cx.render(rsx! {
        div {
            "hello axum! {num}"
            button { onclick: move |_| num += 1, "Increment" }
        }
    })
}