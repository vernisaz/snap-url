extern crate simweb;
mod rand;
mod json;
mod data;
mod gen;
mod snap;

use simweb::WebPage;

const KEY_LEN: usize = 123;

fn main() {
    let web_path = std::env::var(String::from("PATH_INFO"));
    eprintln!{"pi {web_path:?}"}
    match web_path {
        Err(_) => gen::GenPage{}.show(),
        Ok(ref key) if key.len() == (KEY_LEN+1) as usize => snap::SnapPage{key:key.to_string()}.show(),
        _ => Error404{}.show()
    }
}

struct Error404;

impl simweb::WebPage for Error404 {
    fn main_load(&self) -> Result<String, String> {
        Ok(r#"<!doctype html>
<html><body>Page not found</body></html>"#.to_string ())
    }
    
    fn status(&self) -> Option<(u16, &str)> {
        Some((404, "Not found here"))
    }
}