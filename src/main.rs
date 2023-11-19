use eframe::*;
use egui::{panel::Side,Id};
use std::collections::HashMap;
use std::sync::{Arc,Mutex};
mod load;
use load::{load_editor,load_search,load_onglets,load_save_opts,ges_file,show_err_save, show_req_name};
mod utils;


fn main() {
    let opts = NativeOptions{
        initial_window_size:Some(egui::vec2(1024.0, 768.0)),
        transparent:true,
        ..Default::default()
    };
    let mut content = String::new();
    let mut path = String::new();
    let mut file = String::new();
    let mut f_find = true;
    let mut save = false;
    let mut err_save = String::new();
    let mut no_name = false;

    let mut onglets:Arc<Mutex<HashMap<String,String>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut act_ong = String::new();

    let mut clone_ong = onglets.clone();
    // let mut params:(f32,f32);    Rendre resizable la page
    let window = eframe::run_simple_native("Mon IDE", opts, move |ctx,frame|{
        frame.set_centered();
        let s = Side::Left;
        egui::SidePanel::new(s,Id::new("menu")).default_width(150.0).show(ctx,|ui|{
            load_search(ui, &mut f_find, &mut path, &mut file, &mut content, &mut onglets, &mut act_ong);
            ges_file(ui, &f_find, &mut path, &mut content, &mut clone_ong, &mut act_ong);
            ui.separator();
            ui.vertical_centered(|ui|{
                if ui.button("Quit").clicked(){
                    frame.close();
                }        
            });  
        });
        
        egui::CentralPanel::default().show(ctx, |ui|{
            let mut clone_ong = clone_ong.lock().unwrap();
            if clone_ong.len() != 0{
                load_onglets(ui, &mut clone_ong, &mut content, &mut act_ong);
                load_editor(ui, &mut content);
                egui::SidePanel::new(Side::Right,Id::new("opts_f")).default_width(50.0).show(ctx, |ui|{
                    load_save_opts(ui, &mut content, &mut file, &mut save, &mut err_save, &mut no_name, &mut clone_ong, &mut act_ong);
                    show_err_save(ui, &save, &err_save);
                });
            }
            else{
                load_editor(ui, &mut content);
                egui::SidePanel::new(Side::Right,Id::new("opts_f")).default_width(50.0).show(ctx, |ui|{
                    load_save_opts(ui, &mut content, &mut file, &mut save, &mut err_save, &mut no_name, &mut clone_ong, &mut act_ong);
                    show_err_save(ui, &save, &err_save);
                });
            }
            
        });
        
        if no_name{
            show_req_name(ctx, &mut no_name, &mut file, &mut content, &mut save, &mut err_save);
        }
        
        if ctx.input(|k| k.key_pressed(egui::Key::F1)){
                
        }                  
                
    });
            
    match window {
        Ok(_)=>{},
        Err(err)=>{panic!("Erreur critique : {err}")}
    }
}


// Intégrer le changement de dimensions de la fenêtre
// Appliquer le thème

// Développer l'interface pour accroitre les fonctionnalités

// Construire la struct pour regrouper les variables