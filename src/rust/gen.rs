// this script generates a key, attaches a message to it and stores result

use std::convert::TryInto;
use std::time::{SystemTime,UNIX_EPOCH};
use simweb::WebData;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::{Read,SeekFrom,Seek,Write};
use data;
use json;
use std::fs;
use KEY_LEN;
use DATA_DIR;

pub struct GenPage {
    
}

struct DirEntry {
    num: u32,
    time: u64,
    key: [u8;KEY_LEN],
}

impl simweb::WebPage for GenPage {
    fn main_load(&self) -> Result<String, String> {
        let req = WebData::new();
        let _email = req. param("email");
        let ccn = req. param("card-number");
        match ccn {
            Some(_ccn) if _ccn.is_empty() => return Ok("ErrPayment to CC didn't come through".to_owned()),
            // TODO add a payment processing, sending a receipt and texting a security token
            _ => ()
        }
        let load = json::esc_quotes(req. param("message").ok_or("no parameter message")?);
        //let load = esc_quote(load);
        let random_sequence = data::generate_random_sequence(KEY_LEN); // get len as a global constant
        let xx = random_sequence.as_bytes();
        let key = xx[0..KEY_LEN].try_into().unwrap();
        // skip a dir file for quick test
        let hash = data::calc_hash(key);
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let mut file_dat = PathBuf::new();
        file_dat.push(DATA_DIR);
        // how to fix:
        // for every has create hash.dic file, the file contains all active keys and actual .dat files for then
        // its name usually hash-offs.dat where offs is a number not taken yet
        file_dat.push(format!{"{hash}"});
        file_dat.set_extension("dic");
        let mut file = OpenOptions::new().write(true).create(true).read(true).open(&file_dat).map_err(|e| format!{"{e:?}"})?;
        // try to read the current dir first
        let mut buffer = [0; 5]; // Buffer to hold 5 bytes
        let mut buffer2 = [0; 14];
        let mut buffer3 = [0; KEY_LEN];
        let mut clashes = Vec::new();
        let mut max_cla = 0_u32;
        let mut broken_dir = false;
        loop {
            match file.read_exact(&mut buffer) { // read num
                Ok(_) => {
                    match ascii_bytes_to_number(&buffer) {
                        Ok(n) => {
                            match file.read_exact(&mut buffer2) { // read date of creation
                                Ok(_) => {
                                    match ascii_bytes_to_number(&buffer2) {
                                       Ok(time) => {
                                           // can be skeeped if time passed
                                           match file.read_exact(&mut buffer3) {
                                              Ok(_) => {
                                                  if (now.as_millis() as i64) - time < 1000_i64*60*60*24*7  { // 7 days
                                                        // TODO check if key == buffer3 (sanity check)
                                                        let entry = DirEntry{num:n as u32, time: time as u64, key: buffer3.clone()};
                                                        clashes.push(entry)
                                                  }
                                              }
                                              _ => broken_dir = true
                                           }
                                           
                                       }
                                       _ => broken_dir = true
                                    }
                                }
                                _ => broken_dir = true
                            }
                            if n as u32 > max_cla {
                                max_cla = n as u32
                            }
                        }
                        _ => broken_dir = true
                    }
                    
                }
                _ => break
            }
        }
        if broken_dir {
            return Err(format!{"Broken the  clash directory file {file_dat:?}"})
        }
        max_cla += 1;
        // search for first avail slot 
        for avail in 0..max_cla {
            let mut was = false;
            for entry in &clashes {
                if entry.num == avail {
                    was = true;
                    break
                }
            }
            if !was { // use it
                max_cla = avail;
                break
            }
        }
        let entry = DirEntry{num:max_cla, time: now.as_millis() as u64, key: key};
        let num  = entry.num;
        clashes.push(entry);
        let _ = file.seek(SeekFrom::Start(0));
        for entry in clashes {
            let _ = file.write_all(format!{"{:0>5}", entry.num}.as_bytes());
            let _ = file.write_all(format!{"{:0>14}", entry.time}.as_bytes());
            let _ = file.write_all(&entry.key);
            // if errors at writing, then probably notify somehow
        }
        let pos = file.stream_position().map_err(|e| format!{"{e:?}"})?;
        file.set_len(pos).map_err(|e| format!{"{e:?}"})?;
        
        file_dat.pop();
        file_dat.push(format!{"{hash}-{num}"});
        file_dat.set_extension("dat");
        let current = now.as_millis();
        let json = format!{r#"{{"mes": "{load}", "key" : "{random_sequence}", "active":true, "time":{current}, "from":"{}"}}"#,
            std::env::var(String::from("REMOTE_ADDR")).unwrap()
        };
        // assume that te file gets truncated
        fs::write(&file_dat, json).map_err(|e| format!{"{e:?}"})?;

        Ok("Ok".to_owned() + &random_sequence)
    }
}

pub fn ascii_bytes_to_number(bytes: &[u8]) -> Result<i64, Box<dyn std::error::Error>> {
    let s =  std::str::from_utf8(bytes)?;
    let number = s.parse::<i64>()?;
    Ok(number)
}
