#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

//import library for egui 
use eframe::egui;
//Imports libraries use for reading files
use std::fmt::format;
use std::env;
use std::fs;
use std::fmt::Display;

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Christian Guide",
        options,
        Box::new(|_cc| Box::new(MyApp::default())), //runs the app
    );
}


//Variables we use for the app
struct MyApp {
    Search: String,
    dinosaur: String,

    Status: bool,
}




impl Default for MyApp {
    fn default() -> Self {
        Self {
            Search: "".to_owned(),
            dinosaur: "../godsword/test.txt".to_owned(),
            Status: false,
            

        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Conors Christian Guide");
            ui.horizontal(|ui| {
                ui.label("You can find the creator at: ");
                ui.hyperlink("https://conorswebsite.neocities.org/");
                ui.separator();
                
                
            });
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.Search);
            });
           let database = fs::read_to_string(&mut self.dinosaur)
               .expect("god is silent");
            
           if ui.button("Search").clicked() {
               
               self.Status = true
           }

           if self.Status == true {

               if self.Search >= self.dinosaur  {

                   ui.collapsing("Bible explaination for dinosaurs", |ui| {
                       ui.label(format!("{database}"));
                    });

               } else {
                   self.Status = false;
                   ui.label("Topic not found ");

                   
               }
            }

    
    
           

        });
    }
}
