// data layer
#[cfg(test)]
mod rand;
#[cfg(test)]
mod json;

use rand;
use std::time::{UNIX_EPOCH,SystemTime};

/*
trait DataAccess {
    fn read(key: &impl AsRef<str>) -> Option<String>; 
    fn write(key: &impl AsRef<str>, &impl AsRef<str>);
}

pub fn generate_key(size: u8) -> String {
    
}

pub fn get_data_access() -> Result<DataAccess, &&'static str> {
    Err("No data access configured yet")
}
*/
#[allow(dead_code)]
struct SimEntry {
    key: [u8;123],
    stat: u8,
    num: u16
}

pub fn calc_hash(key: [u8;123]) -> u16 {
    let mut res = 0_u16;
    for e in key {
        res += e as u16
    }
    ( res % 256 ) as u16 // TODO make the base value configurable constant
}

pub fn generate_random_sequence(n: usize) -> String {
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                           abcdefghijklmnopqrstuvwxyz\
                           0123456789";
    let mut rng = rand::PCG32::new();
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() % 100_000_000_u128;
    let seq = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() % 240_043_u128;
    rng.seed(seed as u64, seq as u64); // use nano timer or another random source
    let high = charset.len() as f64;
    (0..n)
        .map(|_| {
            let idx = rng.gen_range(0.0,high) as usize;
            charset[idx] as char
        })
        .collect()
}

#[cfg(test)]
use std::fs::{File,self};
#[cfg(test)]
use std::io::{Write,Read};
#[cfg(test)]
use std::convert::TryInto;
#[cfg(test)]
fn main() -> Result<(), std::io::Error> {
    let mut file = File::create(".gen\\snap.dir")?;
    for _ in 0..10 {
        let random_sequence = generate_random_sequence(123);
        println!("Random sequence: {random_sequence}");
        let xx = random_sequence.as_bytes();
        let key = xx[0..123].try_into().unwrap();
        let entry = SimEntry{key:key,stat: 5};
        file.write( &entry.key)?;
        file.write_all(&[entry.stat])?;
        let hash = calc_hash(key);
        let mut file_dat = File::create(format!{".gen\\{hash}.dat"})?;
        let data = format!{r#"{{"key":"{random_sequence}","load":"<div>friend and a family</div>"}}"#};
        file_dat.write(&data.into_bytes())?;
    }
    let mut file = File::open(".gen\\snap.dir")?;
    let mut take = file.take(124_u64); // 123 + 1
    let mut buffer = Vec::new();
    take.read_to_end(&mut buffer)?;
    let key = buffer[0..123].try_into().unwrap();
    println!{"{:?}", json::parse(
         &fs::read_to_string(&format!{".gen\\{}.dat", calc_hash(key)})?)};
    Ok(())
}