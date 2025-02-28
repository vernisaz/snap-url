extern crate simweb;
// this script shows a message or a random content
use std::fs;
use std::convert::TryInto;
use std::path::Path;
use std::time::{SystemTime,UNIX_EPOCH};
use data;
use json;
use json::JsonData::{Data,Text,Num};
use KEY_LEN;

pub struct SnapPage {
    pub key: String
}

impl simweb::WebPage for SnapPage {
    fn main_load(&self) -> Result<String, String> {
        //let req = WebData::new();
        
        let key = self.key.as_bytes()[1..=KEY_LEN].try_into().unwrap();
        let hash = data::calc_hash(key);
        let snap_file = format!{".dat\\{hash}.dat"};
        if Path::new(&snap_file).is_file() {
            let json = json::parse(&fs::read_to_string(&snap_file).map_err(|e| format!{"{e:?}"})?);
            match json {
                Data(ht) => {
                    if let Some(time) = ht.get("time") {
                        if let Num(time) = time {
                            let time = *time as u128;
                            if SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()- time < 1000_u128*60*60*24*7  {
                                if let Some(text) = ht.get("mes") { 
                                    if let Text(text) = text {
                                        // delete the file
                                        fs::remove_file(&snap_file).map_err(|e| format!{"{e:?}"})?; // maybe rewrite with status - active:false
                                        return Ok(text.to_owned())
                                    }
                                }
                            }
                        } else {eprintln!{"no time"}}
                    }
                }
                _ => ()
            }
        } else { eprintln!{"no file {snap_file}"} }
        
        Ok("funny".to_string()) // get random text from web scrapping
    }
}