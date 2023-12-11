use eframe::*;
use egui::{Visuals, vec2};
mod load;
use load::{load_editor,load_close,load_onglets,load_no_name,load_save_opts,show_err_save, show_req_name,show_opts};
mod utils;
mod term;
use term::load_term;
mod obj;
use obj::init;

fn main() {
    let opts = NativeOptions{
        default_theme:Theme::Dark,
        transparent:true,
        ..Default::default()
    };

    let (mut window, mut data, mut term) = init();

    let clone_ong = data.onglets.clone();

    let window = eframe::run_simple_native("Mon IDE", opts, move |ctx,frame|{
        ctx.set_visuals(Visuals::dark());
        let (w,h) = window.get_size();
        frame.set_window_size(vec2(w, h));
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.horizontal(|ui|{
                ui.menu_button("File",|ui|{
                    if ui.button("Open").clicked(){
                        window.act = 1;
                        ui.close_menu()
                    }
                    if ui.button("Save").clicked(){
                        load_save_opts(&mut data.content, &mut data.file, &mut window.save, &mut window.err_save,&mut window.act);  // A revoir
                        ui.close_menu()
                    }
                    if ui.button("Close File").clicked(){
                        load_close(&mut data.file, &data.onglets, &mut data.content, &mut data.act_ong);
                        ui.close_menu()
                    }
                    if ui.button("Options").clicked(){
                        window.act = 3;
                        ui.close_menu()
                    }
                    if ui.button("Quit").clicked(){
                        frame.close()
                    }
                });
                if ui.button("Terminal").clicked(){
                    if term.act{
                        term.act = false;
                    }
                    else{
                        term.act = true;
                    }
                    ui.close_menu()
                }
            });
            ui.separator();
            let mut clone_ong = clone_ong.lock().unwrap();
            if clone_ong.len() != 0{
                load_onglets(ui, &mut clone_ong, &mut data.content, &mut data.act_ong);
                load_editor(ui, &mut data.content,&window.get_size());
            }
            else{
                load_editor(ui, &mut data.content,&window.get_size());
            }
            
        });

        match window.act{
            0=>{},
            1=>{
                load_no_name(ctx, &mut window.f_find, &mut data.path, &mut data.file, &mut data.content, &data.onglets, &mut data.act_ong, &mut window.act);
            },
            2=>{
                show_req_name(ctx, &mut window.act, &mut data.file, &mut data.content, &mut window.save, &mut window.err_save);
            },
            3=>{
                show_opts(ctx, &mut window.get_size(),&mut window.width,&mut window.height,&mut window.err_opts,&mut window.act);
            }
            _=>{}
        }

        if term.act{
            load_term(&ctx, window.save, &window.err_save,&mut term.t, &mut term.t_cmd,&mut term.act,&mut term.curr_path,&window.get_size())
        }                
                
    });
            
    match window {
        Ok(_)=>{},
        Err(err)=>{panic!("Erreur critique : {err}")}
    }
}
