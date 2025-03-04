// this script generates a key, attaches a message to it and stores result

use std::convert::TryInto;
use std::time::{SystemTime,UNIX_EPOCH};
use simweb::WebData;
use std::path::PathBuf;
use data;
use json;
use std::fs;
use KEY_LEN;
use DATA_DIR;

pub struct GenPage {
    
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
       
        let mut file_dat = PathBuf::new();
        file_dat.push(DATA_DIR);
        // how to fix:
        // for every has create hash.col file, the file contains all active keys and actual .dat files for then
        // its name usually hash-offs.dat where offs is a number not taken yet
        file_dat.push(format!{"{hash}"});
        file_dat.set_extension("dat");
        let current = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let json = format!{r#"{{"mes": "{load}", "key" : "{random_sequence}", "active":true, "time":{current}}}"#};
        fs::write(&file_dat, json).map_err(|e| format!{"{e:?}"})?;

        Ok("Ok".to_owned() + &random_sequence)
    }
}
