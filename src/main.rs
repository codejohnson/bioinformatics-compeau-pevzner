mod patterns;
use std::io;
use std::env;
use std::io::prelude::*;
fn readline()->String { io::stdin().lock().lines().next().unwrap().unwrap() }
fn pattern_count() {
    let text = readline();
    let pattern = readline();
    println!("{}", patterns::pattern_count(&text, pattern));
}
fn frecuent_words() {
    let text = readline();
    let k = readline().parse::<usize>();
    if !k.is_err() {
        println!("{:?}", patterns::frecuent_words(text, k.unwrap()));
    }
}

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 || args.len() > 2 {
        println!("this version requires exactly one command line argument.");
        return;
    }
    match &args[1] as &str {
        "-pc" => pattern_count(),
        "-fw" => frecuent_words(),
        &_ => println!("command {} not recognized.",&args[1])
    }
}