#![feature(lazy_cell)]

use std::sync::LazyLock;

use http::HttpConnector;
use tokio::runtime::Runtime;

mod app;
mod directory_tree;
mod http;
mod utilities;

pub static TOKIO: LazyLock<Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
});

pub static HTTP_CONNECTOR: LazyLock<HttpConnector> = LazyLock::new(HttpConnector::new);

#[cfg(target_arch = "wasm32")]
fn main() {
    use app::App;

    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    log::info!("Starting eframe");
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(App::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
