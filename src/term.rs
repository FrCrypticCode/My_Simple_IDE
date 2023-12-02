use std::process::Command;
use std::env::current_exe;
use crate::egui::{Context,Color32};
use crate::show_err_save;

pub fn load_term(ctx:&Context,save:bool,err_save:&String,tes:&mut String,term_cmd:&mut String,act_t:&mut bool){
    egui::TopBottomPanel::bottom("Terminal").show(ctx,|ui|{
        if save{
            show_err_save(ui, &save, &err_save);
        }
        egui::ScrollArea::new([true,true]).show(ui, |ui|{
            ui.set_max_height(50.0);
            let _ = ui.add(egui::TextEdit::multiline(tes)
            .desired_width(700.0)
            .desired_rows(5)
            .code_editor()
            .text_color(Color32::from_rgb(180, 25, 25)))
            
            .interact(egui::Sense { click: false, drag: false, focusable: false });
        });
        ui.separator();
        let term = ui.text_edit_singleline(term_cmd);
        if term_cmd.len() != 0 && term.ctx.input(|k| k.key_down(egui::Key::Enter)){
            // Gestion de l'envoi de cmd
            // Split la cmd puis expédier les parts à use proc
            let rep = use_proc(term_cmd.clone());
            *tes += &term_cmd;
            *tes += "\n";
            *tes += &rep;
            *tes += "\n";
            *term_cmd = String::new(); 
        }
        ui.vertical_centered(|ui|{
            if ui.button("Close").clicked(){
                *act_t = false;
            }
        });
    });
}

fn use_proc(str:String)->String{
    let path = current_exe().unwrap();
    let path = path.parent().unwrap();
    println!("{:?}",path);
    let par = str.split(' ');
    let mut cmd:Vec<&str> = vec![];
    for arg in par{
        cmd.push(arg);
    }
    
    match cmd.len(){
        1=>{
            let proc = Command::new(cmd[0])
                .current_dir(path)
                .output();
            match proc{
                Ok(x)=>{
                    let r = String::from_utf8(x.stdout);
                    match r{
                        Ok(s)=>{return s},
                        Err(err)=>{return err.to_string()}
                    }
                },
                Err(err)=>{
                    return err.to_string()
                }
            }
        },
        2=>{
            let proc = Command::new(cmd[0]).arg(cmd[1])
                .current_dir(path)
                .output();
            match proc{
                Ok(x)=>{
                    let r = String::from_utf8(x.stdout);
                    match r{
                        Ok(s)=>{return s},
                        Err(err)=>{return err.to_string()}
                    }
                },
                Err(err)=>{
                    return err.to_string()
                }
            }
        },
        _=>{return String::from("Veuillez préciser à minima le programme ainsi qu'un seul argument si nécessaire")}
    }
    

}

// Rechercher le dossier actif