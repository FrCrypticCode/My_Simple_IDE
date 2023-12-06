use eframe::*;
use egui::{Visuals, vec2};
use std::collections::HashMap;
use std::sync::{Arc,Mutex};
mod load;
use load::{load_editor,load_close,load_onglets,load_no_name,load_save_opts,show_err_save, show_req_name,show_opts};
mod utils;
mod term;
use term::load_term;


fn main() {
    let opts = NativeOptions{
        default_theme:Theme::Dark,
        transparent:true,
        ..Default::default()
    };
    let mut content = String::new();
    let mut path = String::new();
    let mut file = String::new();
    let mut f_find = true;
    let mut save = false;
    let mut err_save = String::new();
    let mut term = String::new();
    let mut term_cmd = String::new();
    let mut curr_path = std::env::current_dir().unwrap().to_str().unwrap().to_string();
    let mut size_w:(f32,f32) = (1024.0,768.0);
    let mut width = String::new();
    let mut height = String::new();
    let mut err_opts = false;
    let onglets:Arc<Mutex<HashMap<String,Vec<String>>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut act_ong = String::new();
    let mut act_w:u8 = 0;
    let mut act_t = false;

    let clone_ong = onglets.clone();
    // let mut params:(f32,f32);    Rendre resizable la page
    let window = eframe::run_simple_native("Mon IDE", opts, move |ctx,frame|{
        ctx.set_visuals(Visuals::dark());
        let (w,h) = size_w;
        frame.set_window_size(vec2(w, h));
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.horizontal(|ui|{
                ui.menu_button("File",|ui|{
                    if ui.button("Open").clicked(){
                        act_w = 1;
                        ui.close_menu()
                    }
                    if ui.button("Save").clicked(){
                        load_save_opts(&mut content, &mut file, &mut save, &mut err_save,&mut act_w);  // A revoir
                        ui.close_menu()
                    }
                    if ui.button("Close File").clicked(){
                        load_close(&mut file, &onglets, &mut content, &mut act_ong);
                        ui.close_menu()
                    }
                    if ui.button("Options").clicked(){
                        act_w = 3;
                        ui.close_menu()
                    }
                    if ui.button("Quit").clicked(){
                        frame.close()
                    }
                });
                if ui.button("Terminal").clicked(){
                    if act_t{
                        act_t = false;
                    }
                    else{
                        act_t = true;
                    }
                    ui.close_menu()
                }
            });
            ui.separator();
            let mut clone_ong = clone_ong.lock().unwrap();
            if clone_ong.len() != 0{
                load_onglets(ui, &mut clone_ong, &mut content, &mut act_ong);
                load_editor(ui, &mut content,&size_w);
            }
            else{
                load_editor(ui, &mut content,&size_w);
            }
            
        });

        match act_w{
            0=>{},
            1=>{
                load_no_name(ctx, &mut f_find, &mut path, &mut file, &mut content, &onglets, &mut act_ong, &mut act_w);
            },
            2=>{
                show_req_name(ctx, &mut act_w, &mut file, &mut content, &mut save, &mut err_save);
            },
            3=>{
                show_opts(ctx, &mut size_w,&mut width,&mut height,&mut err_opts,&mut act_w);
            }
            _=>{}
        }

        if act_t{
            load_term(&ctx, save, &err_save,&mut term, &mut term_cmd,&mut act_t,&mut curr_path,&size_w)
        }                
                
    });
            
    match window {
        Ok(_)=>{},
        Err(err)=>{panic!("Erreur critique : {err}")}
    }
}

// Construire la struct pour regrouper les variables