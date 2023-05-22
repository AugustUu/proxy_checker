use eframe::egui;
use egui_extras::Column;
use egui_extras::TableBuilder;
use tinyfiledialogs::{self};
use std::{fs::File, io::Read, io::Write};
use eframe::egui::Color32;
use eframe::egui::RichText;
use self::scanner::ProxyResult;
use std::sync::mpsc::{Receiver, Sender};

mod scanner;


pub struct App {
    pub input_proxys: String,
    pub output_proxys: Vec<ProxyResult>,
    tx: Sender<Option<ProxyResult>>,
    rx: Receiver<Option<ProxyResult>>,
    scanning: bool,
}

impl Default for App {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        App{input_proxys: String::new(), output_proxys: Vec::new(),tx,rx, scanning: false }
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        if self.scanning{
            if let Ok(res) = self.rx.try_recv() {

                if res.is_none(){
                    self.scanning = false
                }else{
                    self.output_proxys.push(res.unwrap());
                    //self.output_proxys.push_str(&res.unwrap().ip.to_string());
                    //self.output_proxys.push('\n');
                }
            }
        }

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui,  |ui| {

                if ui.button(RichText::new("▶").color(Color32::from_rgb(68, 137, 178)).size(20.0) ).clicked(){
                    self.scanning = true;
                    self.output_proxys.clear();
                    scanner::scan(&self.input_proxys, self.tx.clone());
                }

                ui.menu_button(RichText::new("File").size(16.0), |ui| {
                    if ui.button("Open").clicked() {
                        if let Some(file_path) = tinyfiledialogs::open_file_dialog("Open File", "./", None){
                            println!("{}",file_path);

                            if let Ok(mut file) = File::open(file_path){
                                if let Err(e) = file.read_to_string(&mut self.input_proxys){
                                    self.input_proxys = e.to_string();
                                }
                            }
                            ui.close_menu();
                        }
                    }
                    if ui.button("Save").clicked() {
                        if let Some(file_path) = tinyfiledialogs::save_file_dialog("Save File", "./",){
                            println!("{}",file_path);
                            let mut output = String::new();
                            for proxy in self.output_proxys.iter(){
                                output.push_str(&proxy.ip.to_string());
                                output.push('\n');
                            }
                            if let Ok(mut file) = File::open(file_path){
                                if let Err(e) = file.write_all(output.as_bytes()) {
                                    println!("Error Writing file\n {}",e);
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

                    ui.horizontal(|ui| {
                        ui.heading("Output");
                        if self.scanning{
                            ui.heading(egui::RichText::new("Scanning").heading().color(egui::Color32::from_rgb(68, 137, 178)));
                            
                        }
                    });

                    let table = TableBuilder::new(ui)
                    .striped(true)
                    .resizable(false)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::remainder())
                    .min_scrolled_height(0.0);
                    
                    
                    //egui::ScrollArea::vertical().show(ui, |ui| {
                        //ui.add(egui::TextEdit::multiline(&mut self.output_proxys).desired_rows(40));
                    table.header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.strong("Latency");
                        });
                        header.col(|ui| {
                            ui.strong("IP");
                        });
                        header.col(|ui| {
                            ui.strong("Port");
                        });
                    }).body(|mut body| {

                        for proxy in self.output_proxys.iter(){
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(RichText::new(proxy.delay.to_string()).size(14.0) );
                                });
                                row.col(|ui| {
                                    if ui.button(RichText::new(proxy.ip.ip().to_string()).size(16.0) ).clicked() {
                                    }
                                });
                                row.col(|ui| {
                                    ui.heading(RichText::new(proxy.ip.port().to_string()).size(16.0) );
                                });
                                
                            });
                        }
                    })
                });
              });

            });

        });
    }

}