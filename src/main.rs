mod gui;

// add table and http, http, socks4, 
//https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/table_demo.rs

#[tokio::main]
async fn main() {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native("Proxy Checker", native_options, Box::new(|cc| Box::new(gui::App::new(cc))));
}

