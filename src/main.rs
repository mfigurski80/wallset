use std::env;

fn main() {
    let args = (env::args().collect::<Vec<String>>()[1..]).to_vec();
    println!("{}", format_url((1920, 1080), &args));
}

fn format_url(size: (u16, u16), keywords: &Vec<String>) -> String {
    return format!(
        "https://source.unsplash.com/random/{}x{}/?{}",
        size.0,
        size.1,
        keywords.join(",")
    );
}
