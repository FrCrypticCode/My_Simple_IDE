use egui::{Ui, Color32, Context,vec2};
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

pub fn load_onglets(ui:&mut Ui,onglets:&mut MutexGuard<'_,HashMap<String,Vec<String>>>,content:&mut String,act_ong:&mut String){
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
                    *content = cont[0].to_string();
                    *act_ong = n_b;
                }
            } 
        }
    });
}

pub fn load_no_name(ctx:&Context,f_find:&mut bool,path:&mut String,file:&mut String,content:&mut String,onglets:&Arc<Mutex<HashMap<String,Vec<String>>>>,act_ong:&mut String,act_w:&mut u8){
    egui::Window::new("Open Options").default_size(vec2(200.0, 150.0)).show(ctx, |ui|{
        load_search(ui,f_find,path,file,content,onglets,act_ong);
        ges_file(ui, &f_find,path,content, onglets,act_ong);
        ui.separator();
        ui.vertical_centered(|ui|{
            if ui.button("Close").clicked(){
                *act_w = 0;
            }        
        }); 
    });
}

pub fn load_save_opts(content:&mut String,file:&mut String,save:&mut bool,err_save:&mut String,act_w:&mut u8){
    if file.len() == 0{
        *act_w = 2;
    }
    else{
        *act_w = 0;
        save_fs(file, content, save, err_save);
    }
}

pub fn load_close(file:&mut String,onglets:&Arc<Mutex<HashMap<String,Vec<String>>>>,content:&mut String,act_ong:&mut String){          
    let nf = file.clone().split("/").last().unwrap().to_string();
    let mut onglets = onglets.lock().unwrap();
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
            *content = c[0].to_string();
            *file = c[1].to_string();
            *act_ong = n.to_string();
            break;
        }
    }
}


fn load_search(ui:&mut Ui,f_find:&mut bool,path:&mut String,file:&mut String,content:&mut String,onglets:&Arc<Mutex<HashMap<String,Vec<String>>>>,act_ong:&mut String){
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
                    let mut arr:Vec<String> = vec![];
                    arr.resize(2, String::new());
                    arr[0] = content.clone();
                    arr[1] = file.clone();
                    list.insert(n.clone(), arr);
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

fn ges_file(ui:&mut Ui,f_find:&bool,path:&mut String,content:&mut String,onglets:&Arc<Mutex<HashMap<String,Vec<String>>>>,act_ong:&mut String){
    if !f_find{
        ui.colored_label(Color32::from_rgb(255,0,0), "Fichier non trouvé.");
        if ui.button("Créer le fichier ?").clicked(){
            match fs::File::create(&path){
                Ok(_)=>{
                    let n = path.split("/").last().unwrap().to_string();
                    let c = content.clone().to_string();
                    let mut list = onglets.lock().unwrap();
                    let mut arr:Vec<String> = vec![];
                    arr.resize(2, String::new());
                    arr[0] = c;
                    arr[1] = path.clone();
                    list.insert(n.clone(), arr);
                    *act_ong = n;
                },
                Err(_err)=>{}
            }
        }
    }
}

pub fn show_req_name(ctx:&Context,no_name:&mut u8,file:&mut String,content:&mut String,save:&mut bool,err_save:&mut String){
    egui::Window::new("Need name").show(ctx, |ui|{
        ui.text_edit_singleline(file);
        ui.horizontal(|ui|{
            if ui.button("Confirm").clicked(){
                save_fs(&file,content,save,err_save);
                *no_name = 0;
            }
            if ui.button("Abort").clicked(){
                *no_name = 0;
            }
        });
        
    });
}

pub fn show_err_save(ui:&mut Ui,save:&bool,err_save:&String){
    if err_save.len() != 0 && *save == false{
        ui.colored_label(Color32::from_rgb(255,0,0), err_save);
    }
}