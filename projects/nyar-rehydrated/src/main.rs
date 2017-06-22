#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

use axum::{response::Html, routing::get, Router};
use dioxus::prelude::*;

#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(Router::new().route("/:package/*modules", get(app_endpoint)).into_make_service())
        .await
        .unwrap();
}

async fn app_endpoint() -> Html<String> {
    // create a VirtualDom with the app component
    let mut app = VirtualDom::new(app);
    // rebuild the VirtualDom before rendering
    let _ = app.rebuild();
    // render the VirtualDom to HTML
    Html(dioxus_ssr::render_vdom(&app))
}

// create a component that renders a div with the text "hello world"
fn app(cx: Scope) -> Element {
    cx.render(rsx!(div { "hello world" }))
}
