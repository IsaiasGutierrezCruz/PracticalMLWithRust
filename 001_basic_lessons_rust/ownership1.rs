fn main(){
    let lang = "rust";
    let rust1 = add_version(&lang);
    println!("lang is {:?}", rust1);
}

fn add_version(s: &str) -> String {
    s.to_string() + " 2018."
}