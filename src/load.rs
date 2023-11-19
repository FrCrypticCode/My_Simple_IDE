use egui::{Ui, Color32, Context};
use std::{fs,sync::{Arc,Mutex, MutexGuard},collections::HashMap};

use crate::utils::save_fs;

pub fn load_editor(ui:&mut Ui,content:&mut String){
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

pub fn load_onglets(ui:&mut Ui,onglets:&mut MutexGuard<'_,HashMap<String,String>>,content:&mut String,act_ong:&mut String){
    ui.horizontal(|ui|{
        for ong in onglets.iter(){
            let (name,cont) = ong;
            if *act_ong == *name{
                    ui.colored_label(Color32::from_rgb(50, 50, 50), act_ong.clone());
            }
            else{
                let n_b = name.clone();
                let x = ui.add(egui::Button::new(&n_b));
                if x.clicked(){
                    *content = cont.to_string();
                    *act_ong = n_b;
                }
            } 
        }
    });
}


pub fn load_save_opts(ui:&mut Ui,content:&mut String,file:&mut String,save:&mut bool,err_save:&mut String,no_name:&mut bool,onglets:&mut MutexGuard<'_,HashMap<String,String>>,act_ong:&mut String){
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
            
            let nf = file.clone().split("/").last().unwrap().to_string();
            for key in onglets.clone().into_keys(){
                if key == nf{
                    onglets.remove(&key);
                }
            }
            if onglets.is_empty(){
                *content = String::new();
                *file = String::from("");
                *act_ong = String::new();
            }
            else{
                for l in onglets.iter(){
                    let (n,c) = l;
                    *content = c.to_string();
                    *file = n.to_string();
                    *act_ong = n.to_string();
                    break;
                }
            }
            
        }
    });
}

pub fn load_search(ui:&mut Ui,f_find:&mut bool,path:&mut String,file:&mut String,content:&mut String,onglets:&Arc<Mutex<HashMap<String,String>>>,act_ong:&mut String){
    ui.label("Chemin de votre fichier");
    ui.text_edit_singleline(path);
    if path.len()>0{
        if ui.button("Open").clicked(){
            match fs::read_to_string(&path){
                Ok(x)=>{
                    *file = path.clone();
                    *path = String::from("");
                    *content = x;
                    let n = file.clone().split("/").last().unwrap().to_string();
                    let mut list = onglets.lock().unwrap();
                    list.insert(n.clone(), content.clone());
                    *act_ong = n;
                    *f_find = true;
                },
                Err(_err)=>{
                    *f_find = false;
                }   
            }
        }
    }
}

pub fn ges_file(ui:&mut Ui,f_find:&bool,path:&mut String,content:&mut String,onglets:&Arc<Mutex<HashMap<String,String>>>,act_ong:&mut String){
    if !f_find{
        ui.colored_label(Color32::from_rgb(255,0,0), "Fichier non trouvé.");
        if ui.button("Créer le fichier ?").clicked(){
            match fs::File::create(&path){
                Ok(_)=>{
                    let n = path.split("/").last().unwrap().to_string();
                    let c = content.clone().to_string();
                    let mut list = onglets.lock().unwrap();
                    list.insert(n.clone(), c);
                    *act_ong = n;
                },
                Err(_err)=>{}
            }
        }
    }
}

pub fn show_req_name(ctx:&Context,no_name:&mut bool,file:&mut String,content:&mut String,save:&mut bool,err_save:&mut String){
    egui::Window::new("Need name").show(ctx, |ui|{
        ui.text_edit_singleline(file);
        if ui.button("Confirm").clicked(){
            save_fs(&file,content,save,err_save);
            *no_name = false;
        }
    });
}

pub fn show_err_save(ui:&mut Ui,save:&bool,err_save:&String){
    if err_save.len() != 0 && *save == false{
        ui.colored_label(Color32::from_rgb(255,0,0), err_save);
    }
}