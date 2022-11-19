#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

//import library for egui 
use eframe::egui;
//Imports libraries use for reading files
use std::fs;

//Main where eframe is launched 
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


//Variables we set the values of the vars  for the impl Default
struct MyApp {
    search: String,
    dinosaur: String,
    languages: String,
    status: u32,
}



//Allows variables to be available all over 
impl Default for MyApp {
    fn default() -> Self {
        Self {
            search: "".to_owned(), //search var we use later
            dinosaur: "../godsword/test.txt".to_owned(),
            status: 1, //sets status to the number 1 
            languages: "../godsword/languages.txt".to_owned(),

        }
    }
}
//The eframe impl type that setsup the layout of the ui 
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(egui::Color32::GOLD); //changes text to
                                                                             //color gold
            ui.style_mut().wrap = Some(false);
            ui.heading("Conors Christian Guide"); //The title
            ui.horizontal(|ui| {
                ui.label("You can find the creator at: "); 
                ui.hyperlink("https://conorswebsite.neocities.org/");
                ui.separator(); //it is the |
                
                
            });
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.search); //The search bar
            });
            //reads test.txt file
           let database = fs::read_to_string(&mut self.dinosaur)
               .expect("test.txt NOT FOUND!!!"); //if text file is not found
            
           let langdata = fs::read_to_string(&mut self.languages)
               .expect("File languages NOT FOUND!! ");
           //launches the search and compares it to whatever in loop
           if self.status == 1 {

               if self.search == "dinosaur" {

                   ui.collapsing("Bible explaination for dinosaurs", |ui| {
                       ui.label(format!("{database}"));
                    });

               } if self.search == "languages" {
                   ui.collapsing("Why is there multiple languages?", |ui| {
                       ui.label(format!("{langdata}"));
                    });

               } else {
                   let _ = self.status - 1;
                   

                   
               }
            }

    
    
           

        });
    }
}
