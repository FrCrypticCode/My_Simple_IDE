use std::fs;

pub fn save_fs(file:&String,content:&mut String,save:&mut bool,err_save:&mut String){
    match fs::write(&file, content.clone()){
        Ok(_)=>{*save = true;},
        Err(err)=>{
            *save = false;
            *err_save = err.to_string();
        }
    }
}