extern crate curl;
use curl::http;

mod rhyme;
use rhyme::*;

fn main() {
    let resp = http::handle()
        .get("http://rhymebrain.com/talk?function=getRhymes&word=heart")
        .exec().unwrap();
}
