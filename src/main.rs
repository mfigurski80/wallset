extern crate reqwest;
use std::fs::File;
use std::{env, io};
use wallpaper;

fn main() {
    let args = (env::args().collect::<Vec<String>>()[1..]).to_vec();
    let path_to_write = "/home/miko/Pictures/Wallpapers/auto.jpg";
    get_image(format_url((1920, 1080), &args), path_to_write);
    set_wallpaper(path_to_write);
}

fn set_wallpaper(path: &str) {
    let orig = wallpaper::get().unwrap();
    println!("Original paper: {:?}", orig);
    wallpaper::set_from_path(path).expect("wallpaper set failed");
    wallpaper::set_mode(wallpaper::Mode::Crop).expect("wallpaper mode set failed");
    println!("New wallpaper:  {:?}", wallpaper::get().unwrap());
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
}
