extern crate reqwest;
use std::fs::File;
use std::{env, io};

fn main() {
    let args = (env::args().collect::<Vec<String>>()[1..]).to_vec();
    get_image(format_url((1920, 1080), &args), "./image.jpg");
    // println!("{}", format_url((1920, 1080), &args));
}

fn format_url(size: (u16, u16), keywords: &Vec<String>) -> String {
    return format!(
        "https://source.unsplash.com/random/{}x{}/?{}",
        size.0,
        size.1,
        keywords.join(",")
    );
}

fn get_image(url: String, file_name: &str) {
    let mut resp = reqwest::blocking::get(url).expect("request failed");
    let mut out = File::create(file_name).expect("file creation failed");
    io::copy(&mut resp, &mut out).expect("file copy failed");
    // println!("{:?}", resp);
}
