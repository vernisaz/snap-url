extern crate simweb;
// this script shows a message or a random content
use std::fs;
use std::convert::TryInto;
use std::path::{Path,PathBuf};
use std::time::{SystemTime,UNIX_EPOCH};
use std::fs::OpenOptions;
use std::io::Read;
use data;
use json;
use gen;
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
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let mut snap_file = PathBuf::new();
        let mut entry_num = None;
        snap_file.push(DATA_DIR);
        // TODO name will be taken from hash.col file
        snap_file.push(format!{"{hash}"});
        snap_file.set_extension("dic");
        // read the hash clash directory first
        if Path::new(&snap_file).is_file() {
            let mut file = OpenOptions::new().read(true).open(&snap_file).map_err(|e| format!{"{e:?}"})?;
            let mut num_buf = [0; 5]; // Buffer to hold 5 bytes
            let mut time_buf = [0; 14];
            let mut key_buf = [0; KEY_LEN];
            loop {
                match file.read_exact(&mut num_buf) { // read num
                    Ok(_) => {
                        match gen::ascii_bytes_to_number(&num_buf) {
                            Ok(n) => {
                                match file.read_exact(&mut time_buf) { // read date of creation
                                    Ok(_) => {
                                        match gen::ascii_bytes_to_number(&time_buf) {
                                           Ok(time) => {
                                               // can be skeeped if time passed
                                               match file.read_exact(&mut key_buf) {
                                                    Ok(_) => {
                                                        if key == key_buf { // the match
                                                            if (now.as_millis() as i64)- time < 1000_i64*60*60*24*7  { // 7 days
                                                                    entry_num = Some(n)
                                                            }
                                                            break
                                                        }
                                                    }
                                                    _ => break
                                               }
                                           }
                                           _ => break
                                        }
                                    }
                                    _ => break
                                }
                            }
                            _ => break
                        }
                    }
                    _ => break
                }
            }
        }
        snap_file.pop();
        if let Some(num) = entry_num {
            snap_file.push(format!{"{hash}-{num}"})
        } else {
            snap_file.push(format!{"{hash}"})
        }
        snap_file.set_extension("dat");
        if Path::new(&snap_file).is_file() {
            let json = json::parse(&fs::read_to_string(&snap_file).map_err(|e| format!{"{e:?}"})?);
            match json {
                Data(ht) => {
                    if let Some(time) = ht.get("time") {
                        if let Num(time) = time {
                            let time = *time as u128;
                            if now.as_millis()- time < 1000_u128*60*60*24*7  {
                                if let Some(text) = ht.get("mes") { 
                                    if let Text(text) = text {
                                        // delete the file
                                        fs::remove_file(&snap_file).map_err(|e| format!{"{e:?}"})?; // maybe rewrite with status - active:false
                                        return Ok(text.to_owned())
                                    }
                                }
                            } else {
                                fs::remove_file(&snap_file).map_err(|e| format!{"{e:?}"})?;
                            }
                        } else {eprintln!{"no time"}}
                    }
                }
                _ => ()
            }
        } else { eprintln!{"no file {snap_file:?}"} }
        let mut ran = PCG32::new ();
        let seed = now.as_nanos() % 100_000_000_u128;
        let seq = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() % 362_043_u128;
        ran.seed(seed as u64, seq as u64);
    
        let slot = ran.next_u32() % 32;
        let mut snap_file = PathBuf::new();
        snap_file.push(FAKE_DIR);
        snap_file.push(format!{"{slot}"});
        snap_file.set_extension("scr");
        if Path::new(&snap_file).is_file() {
            Ok(fs::read_to_string(&snap_file).map_err(|e| format!{"{e:?}"})?)
        } else { // just read some random file from there
            let path = Path::new(FAKE_DIR);
            let mut count = 0;
        
            if path.is_dir() {
                for entry in fs::read_dir(path).map_err(|e| format!{"{e:?}"})? {
                    let entry = entry.map_err(|e| format!{"{e:?}"})?;
                    if entry.path().is_file() {
                        count += 1;
                    }
                }
                let ran_val = ran.gen_range(1.0, count as f64) as u32;
                eprintln!{"found {count} entries in {path:?} and ran {ran_val}"}
                count = 0;
                for entry in fs::read_dir(path).map_err(|e| format!{"{e:?}"})? {
                    let entry = entry.map_err(|e| format!{"{e:?}"})?;
                    if entry.path().is_file() {
                        count += 1;
                        if count == ran_val {
                            return  Ok(fs::read_to_string(entry.path()).map_err(|e| format!{"{e:?}"})?)
                        }
                    }
                }
            }
            
            Ok(format!{"funny {slot}"}) // get random text from web scrapping
        }
    }
}