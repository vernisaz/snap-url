// this script generates a key, attaches a message to it and stores result

use std::convert::TryInto;
use std::time::{SystemTime,UNIX_EPOCH};
use simweb::WebData;
use data;
use json;
use std::fs;
use KEY_LEN;

pub struct GenPage {
    
}

impl simweb::WebPage for GenPage {
    fn main_load(&self) -> Result<String, String> {
        let req = WebData::new();
        let load = json::esc_quotes(req. param("message").ok_or("no parameter message")?);
        //let load = esc_quote(load);
        let random_sequence = data::generate_random_sequence(KEY_LEN); // get len as a global constant
        let xx = random_sequence.as_bytes();
        let key = xx[0..KEY_LEN].try_into().unwrap();
        // skip a dir file for quick test
        let hash = data::calc_hash(key);
        let file_dat = &format!{".dat\\{hash}.dat"};
        let current = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let json = format!{r#"{{"mes": "{load}", "key" : "{random_sequence}", "active":true, "time":{current}}}"#};
        fs::write(&file_dat, json).map_err(|e| format!{"{e:?}"})?;

        Ok("Ok".to_owned() + &random_sequence)
    }
}
