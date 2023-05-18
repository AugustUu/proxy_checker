use eframe::egui;
use tinyfiledialogs::{self};
use std::{fs::File, io::Read};
use eframe::egui::Color32;
use eframe::egui::RichText;
use self::scanner::ProxyResult;
use std::sync::mpsc::{Receiver, Sender};
mod scanner;


pub struct App {
    pub input_proxys: String,
    pub output_proxys: Vec<ProxyResult>,
    tx: Sender<ProxyResult>,
    rx: Receiver<ProxyResult>,
}

impl Default for App {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        App{input_proxys: "".to_string(), output_proxys: Vec::new(),tx,rx }
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        if let Ok(res) = self.rx.try_recv() {
            println!("{}",res.ip)
        }

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui,  |ui| {

                if ui.button(RichText::new("▶").color(Color32::from_rgb(68, 137, 178,)).size(20.0) ).clicked(){
                    scanner::scan(&self.input_proxys, self.tx.clone());
                }

                ui.menu_button(RichText::new("File").size(16.0), |ui| {
                    if ui.button("Open").clicked() {
                        if let Some(file_path) = tinyfiledialogs::open_file_dialog("Open", "./", None){
                            println!("{}",file_path);

                            if let Ok(mut file) = File::open(file_path){
                                if let Err(e) = file.read_to_string(&mut self.input_proxys){
                                    self.input_proxys = e.to_string();
                                }
                            }
                            ui.close_menu();
                        }

                    }
                });

            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.horizontal_centered(|ui|{

              ui.vertical(|ui|{
                  ui.heading("Input");

                  egui::ScrollArea::vertical().show(ui, |ui| {
                      ui.add(egui::TextEdit::multiline(&mut self.input_proxys).desired_rows(40));
                  });
              });

              ui.separator();

              ui.push_id(ui.next_auto_id(), |ui| {
                  ui.vertical(|ui| {
                      ui.heading("Output");


                  });
              });

            });

        });
    }

}