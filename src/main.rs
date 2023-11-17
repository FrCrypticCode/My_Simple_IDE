use eframe::*;
use egui::{Color32};
use std::fs;

fn main() {
    let opts = NativeOptions{
        initial_window_size:Some(egui::vec2(1024.0, 768.0)),
        default_theme: Theme::Dark,
        ..Default::default()
    };
    let mut content = String::new();
    let mut path = String::new();
    let mut file = String::new();
    let mut f_find = true;
    let mut save = false;
    let mut err_save = String::new();
    let window = eframe::run_simple_native("Mon IDE", opts, move |ctx,frame|{
            egui::CentralPanel::default().show(ctx, |ui|{
                ui.label("My Code Editor");
                ui.columns(2, |ui|{
                    // Colonne Gauche                   
                    let c = ui[0].code_editor(&mut content).scroll_to_me(Some(egui::Align::Center));

                    // Colonne Droite
                    ui[1].label("Options");
                    ui[1].horizontal(|ui|{
                        if content.len()>0 && file.len() != 0{
                            if ui.button("Save").clicked(){
                                match fs::write(&file, content.clone()){
                                    Ok(_)=>{save = true;},
                                    Err(err)=>{
                                        save = false;
                                        err_save = err.to_string();
                                    }
                                }
                            }
                            if ui.button("Close").clicked(){
                                content = String::new();
                                file = String::from("");
                            }
                        }
                    });
                });
                ui.separator();
                ui.label("Chemin de votre fichier");
                ui.horizontal(|ui|{
                    ui.text_edit_singleline(&mut path);
                    if path.len()>0{
                        if ui.button("Open").clicked(){
                            match fs::read_to_string(&path){
                                Ok(x)=>{
                                    file = path.clone();
                                    path = String::from("");
                                    content = x;
                                    f_find = true;
                                },
                                Err(_err)=>{
                                    f_find = false;
                                }   
                            }
                        }
                    }
                });
                if !f_find{
                    ui.colored_label(Color32::from_rgb(255,0,0), "Fichier non trouvé.");
                    if ui.button("Créer le fichier ?").clicked(){
                        match fs::File::create(&path){
                            Ok(_)=>{},
                            Err(err)=>{}
                        }
                    }
                }
                if err_save.len() != 0 && save == false{
                    ui.colored_label(Color32::from_rgb(255,0,0), &err_save);
                }
                ui.separator();
                ui.vertical_centered(|ui|{
                    if ui.button("Quit").clicked(){
                        frame.close();
                    }
                    
                });
            });
            if ctx.input(|k| k.key_pressed(egui::Key::F1)){
                
            }         
    });
    match window {
        Ok(_)=>{},
        Err(err)=>{panic!("Erreur critique : {err}")}
    }
}


// Revoir la gestion des résultats Erreurs et Succès
// Travailler la disposition de l'éditeur
