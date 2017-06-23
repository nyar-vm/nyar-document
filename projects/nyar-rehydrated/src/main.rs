#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

use axum::{
    extract::{Path, Query},
    response::Html,
    routing::get,
    Router,
};
use dioxus::prelude::*;
use dioxus_ssr::Renderer;
use nyar_document::{PackageContext, PackagePage, PackageQuery};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(
            Router::new()
                // .route("/@api", get(module_endpoint))
                .route("/:package", get(package_endpoint))
                .route("/:package/*modules", get(module_endpoint))
                .into_make_service(),
        )
        .await
        .unwrap();
}

#[derive(Clone, Debug, PartialEq, Props)]
pub struct ModuleQuery {
    package: String,
    modules: Vec<String>,
}

pub async fn package_endpoint(Path(package): Path<String>, Query(query): Query<PackageQuery>) -> Html<String> {
    let mut renderer = Renderer::new();
    renderer.pre_render = true;
    let mut vdom = VirtualDom::new_with_props(PackagePage, PackageContext::new(package) + query);
    let _ = vdom.rebuild();
    let html = renderer.render(&vdom);
    Html(html)
}

pub async fn module_endpoint(Path((package, modules)): Path<(String, String)>) -> Html<String> {
    let mut renderer = Renderer::new();
    renderer.pre_render = true;
    let mut vdom = VirtualDom::new_with_props(
        module_render,
        ModuleQuery { package, modules: modules.split("/").map(ToString::to_string).collect() },
    );
    let _ = vdom.rebuild();
    let html = renderer.render(&vdom);
    Html(html)
}

// create a component that renders a div with the text "hello world"
pub fn module_render(cx: Scope<ModuleQuery>) -> Element {
    let pkg = &cx.props.package;
    let module = cx.props.modules.join("::");

    cx.render(rsx!(div {
        "hello module"
        div {
            "{pkg}"
        }
        div {
            "{module}"
        }
    }))
}
