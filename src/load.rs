use egui::{Ui, Color32};
use std::fs;

use crate::utils::save_fs;

pub fn load_editor(ui:&mut Ui,content:&mut String){
    ui.set_height(700.0);
    egui::ScrollArea::new([true,true]).show(ui, |ui|{
        ui.add(
            egui::TextEdit::multiline(content)
            .font(egui::TextStyle::Body)
            .code_editor()
            .desired_width(700.0)
            .desired_rows(50)
        );
    });
}

pub fn load_save_opts(ui:&mut Ui,content:&mut String,file:&mut String,save:&mut bool,err_save:&mut String,no_name:&mut bool){
    ui.label("Options");
    ui.horizontal(|ui|{
        if ui.button("Save").clicked(){
            if file.len() == 0{
                *no_name = true;
            }
            else{
                *no_name = false;
                save_fs(file, content, save, err_save);
            }
        }
        if ui.button("Close").clicked(){
            *content = String::new();
            *file = String::from("");
        }
    });
}

pub fn load_search(ui:&mut Ui,f_find:&mut bool,path:&mut String,file:&mut String,content:&mut String){
    ui.label("Chemin de votre fichier");
    ui.text_edit_singleline(path);
    if path.len()>0{
        if ui.button("Open").clicked(){
            match fs::read_to_string(&path){
                Ok(x)=>{
                    *file = path.clone();
                    *path = String::from("");
                    *content = x;
                    *f_find = true;
                },
                Err(_err)=>{
                    *f_find = false;
                }   
            }
        }
    }
}

pub fn ges_file(ui:&mut Ui,f_find:&bool,path:&mut String){
    if !f_find{
        ui.colored_label(Color32::from_rgb(255,0,0), "Fichier non trouvé.");
        if ui.button("Créer le fichier ?").clicked(){
            match fs::File::create(&path){
                Ok(_)=>{},
                Err(_err)=>{}
            }
        }
    }
}

pub fn show_err_save(ui:&mut Ui,save:&bool,err_save:&String){
    if err_save.len() != 0 && *save == false{
        ui.colored_label(Color32::from_rgb(255,0,0), err_save);
    }
}