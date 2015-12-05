extern crate curl;
mod rhyme;

use curl::http;
use rhyme::*;
use std::env;

fn main() {
    let word = env::args().nth(1).unwrap_or("heart".to_string());
    let url = format!("http://rhymebrain.com/talk?function=getRhymes&word={}", word);
    let resp = http::handle().get(url).exec().unwrap();

    println!("{}", resp);
}
