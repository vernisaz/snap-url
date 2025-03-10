extern crate simweb;
// this script shows a message or a random content
use std::fs;
use std::convert::TryInto;
use std::path::{Path,PathBuf};
use std::time::{SystemTime,UNIX_EPOCH};
use data;
use json;
use json::JsonData::{Data,Text,Num};
use KEY_LEN;
use {DATA_DIR, FAKE_DIR};
use rand::PCG32;

pub struct SnapPage {
    pub key: String
}

impl simweb::WebPage for SnapPage {
    fn main_load(&self) -> Result<String, String> {
        //let req = WebData::new();
        
        let key = self.key.as_bytes()[1..=KEY_LEN].try_into().unwrap();
        let hash = data::calc_hash(key);
        let mut snap_file = PathBuf::new();
        snap_file.push(DATA_DIR);
        // TODO name will be taken from hash.col file
        snap_file.push(format!{"{hash}"});
        snap_file.set_extension("dat");
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
        } else { eprintln!{"no file {snap_file:?}"} }
        let mut ran = PCG32::new ();
        // TODO set seed from time
        let slot = ran.next_u32() % 32;
        let mut snap_file = PathBuf::new();
        snap_file.push(FAKE_DIR);
        snap_file.push(format!{"{slot}"});
        snap_file.set_extension("scr");
        if Path::new(&snap_file).is_file() {
            Ok(fs::read_to_string(&snap_file).map_err(|e| format!{"{e:?}"})?)
        } else {
            Ok(format!{"funny {slot}"}) // get random text from web scrapping
        }
    }
}