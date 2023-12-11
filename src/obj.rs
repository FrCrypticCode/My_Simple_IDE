use std::sync::{Arc,Mutex};
use std::collections::HashMap;
pub struct Window{
    size:(f32,f32),
    pub width:String,
    pub height:String,
    pub f_find:bool,
    pub save:bool,
    pub err_save:String,
    pub err_opts:bool,
    pub act:u8
}
impl Window{
    fn new()->Window{
        return Window { 
            size: (1024.0,768.0), 
            width: String::new(), 
            height: String::new(),
            f_find: true,
            save: false,
            err_save: String::new(), 
            err_opts: false, 
            act: 0 
        }
    }
    pub fn get_size(&self)->(f32,f32){
        return self.size
    }
}

pub struct Data{
    pub onglets:Arc<Mutex<HashMap<String,Vec<String>>>>,
    pub act_ong:String,
    pub content:String,
    pub path:String,
    pub file:String
}
impl Data{
    fn new()->Data{
        return Data {
            onglets: Arc::new(Mutex::new(HashMap::new())), 
            act_ong: String::new(),
            content: String::new(),
            path: String::new(),
            file: String::new() 
        }
    }
}

pub struct Term{
    pub t:String,
    pub t_cmd:String,
    pub curr_path:String,
    pub act:bool
}
impl Term{
    fn new()->Term{
        return Term { 
            t: String::new(),
            t_cmd: String::new(),
            curr_path: std::env::current_dir().unwrap().to_str().unwrap().to_string(),
            act: false
        }
    }
}

pub fn init()->(Window,Data,Term){
    return (Window::new(),Data::new(),Term::new())
}