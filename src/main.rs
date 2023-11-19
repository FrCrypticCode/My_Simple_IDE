use eframe::*;
use egui::{panel::Side,Id};

mod load;
use load::{load_editor,load_search,load_save_opts,ges_file,show_err_save};
mod utils;
use utils::save_fs;


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
    // let mut params:(f32,f32);    Rendre resizable la page
    let window = eframe::run_simple_native("Mon IDE", opts, move |ctx,frame|{
        frame.set_centered();
        let s = Side::Left;
        egui::SidePanel::new(s,Id::new("menu")).default_width(150.0).show(ctx,|ui|{
            load_search(ui, &mut f_find, &mut path, &mut file, &mut content);
            ges_file(ui, &f_find, &mut path);
            ui.separator();
            ui.vertical_centered(|ui|{
                if ui.button("Quit").clicked(){
                    frame.close();
                }        
            });  
        });
        
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.horizontal(|ui|{
                load_editor(ui, &mut content);
                show_err_save(ui, &save, &err_save);
            });
            egui::SidePanel::new(Side::Right,Id::new("opts_f")).default_width(50.0).show(ctx, |ui|{
                load_save_opts(ui, &mut content, &mut file, &mut save, &mut err_save,&mut no_name);
            });
        });
        
        if no_name{
            egui::Window::new("Need name").show(ctx, |ui|{
                ui.text_edit_singleline(&mut file);
                if ui.button("Confirm").clicked(){
                    save_fs(&file,&mut content, &mut save,&mut err_save);
                    no_name = false;
                }
            });
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
// Intégrer des onglets pour chaque fichier ouvert
// Corriger les Warnings
// Développer l'interface pour accroitre les fonctionnalités