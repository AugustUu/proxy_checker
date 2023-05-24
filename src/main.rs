use eframe::{epaint::Vec2, NativeOptions};

mod gui;

// add table and http, http, socks4,
//https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/table_demo.rs

#[tokio::main]
async fn main() {
    let native_options = NativeOptions {
        resizable: false,
        max_window_size: Some(Vec2::new(630.0, 635.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Proxy Checker",
        native_options,
        Box::new(|cc| Box::new(gui::App::new(cc))),
    );
}
