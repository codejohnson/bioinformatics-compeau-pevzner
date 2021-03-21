mod patterns;
use std::io;
use std::env;
use std::io::prelude::*;
fn readline()->String { io::stdin().lock().lines().next().unwrap().unwrap() }
fn pattern_count() {
    let text = readline();
    let pattern = readline();
    println!("{}", patterns::pattern_count(&text, &pattern));
}
fn most_frequent_words() {
    let text = readline();
    let k = readline().parse::<usize>();
    if !k.is_err() {
        println!("{:?}", patterns::most_frequent_words(&text, k.unwrap()));
    }
}
fn reverse_complement() {
    let text = readline();
    println!("{:?}", patterns::reverse_complement(text));
}
fn pattern_matching() {
    let text = readline();
    let pattern = readline();
    for pos in patterns::pattern_matching(&text,pattern).iter() {
        print!("{} ", pos);
    }
    println!();
}
fn approximate_pattern_matching() {
    let text = readline();
    let pattern = readline();
    let d = readline().parse::<u32>();
    for pos in patterns::approximate_pattern_matching(&text,pattern,d.unwrap()).iter() {
        print!("{} ", pos);
    }
    println!();
}

fn minimum_skew() {
    let text = readline();
    let minskews = patterns::minimum_skew(&text);
    println!("{:?}", minskews);
}
fn hamming_distance() {
    let s = readline();
    let t = readline();
    let distance = patterns::hamming_distance(&s, &t);
    println!("{:?}", distance);
}

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 || args.len() > 2 {
        println!("this version requires exactly one command line argument.");
        return;
    }
    match &args[1] as &str {
        "-pc" => pattern_count(),
        "-fw" => most_frequent_words(),
        "-rc" => reverse_complement(),
        "-pm" => pattern_matching(),
        "-apm" => approximate_pattern_matching(),
//        "-cf" => clump_finding(), 
        "-ms" => minimum_skew(),
        "-hd" => hamming_distance(),
&_ => println!("command {} not recognized.",&args[1])
    }
}