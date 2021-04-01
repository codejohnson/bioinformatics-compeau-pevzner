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

fn find_frequent_word_with_missmatch_by_sorting() {
    let text = readline();
    let values = readline();
    let split = values.split(' ').collect::<Vec<&str>>();
    let k = split[0].parse::<usize>().unwrap();
    let d = split[1].parse::<usize>().unwrap();
    println!("{:?}", patterns::find_frequent_word_with_missmatch_by_sorting(&text, k, d));
}

fn find_frequent_word_with_missmatch_and_reverse_by_sorting() {
    let text = readline();
    let values = readline();
    let split = values.split(' ').collect::<Vec<&str>>();
    let k = split[0].parse::<usize>().unwrap();
    let d = split[1].parse::<usize>().unwrap();
    println!("{:?}", patterns::find_frequent_word_with_missmatch_and_reverse_by_sorting(&text, k, d));
}


fn find_frequent_word_with_missmatch() {
    let text = readline();
    let values = readline();
    let split = values.split(' ').collect::<Vec<&str>>();
    let k = split[0].parse::<usize>().unwrap();
    let d = split[1].parse::<usize>().unwrap();
    println!("{:?}", patterns::find_frequent_word_with_missmatch(&text, k, d));
}


fn reverse_complement() {
    let text = readline();
    println!("{:?}", patterns::reverse_complement(&text));
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
    let d = readline().parse::<usize>();
    for pos in patterns::approximate_pattern_matching(&text,pattern,d.unwrap()).iter() {
        print!("{} ", pos);
    }
    println!();
}

fn clump_finding() {
    let text = readline();
    let arguments = readline();
    let values = arguments.split(" ").collect::<Vec<&str>>();
    let k_size =values[0].parse::<usize>().unwrap();
    let lwin =values[1].parse::<usize>().unwrap();
    let times =values[2].parse::<usize>().unwrap();
    let clump_patters = patterns::clump_finding(&text, k_size, lwin, times);
    println!("{:?}", clump_patters);
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
fn pattern_to_number() {
    let pttrn = readline();
    let num = patterns::pattern_to_number(&pttrn);
    println!("{:?}", num);
}
fn neighbors() {
    let pttrn = readline();
    let d = readline().parse::<usize>().unwrap();
    let pttrns = patterns::neighbors(&pttrn,d);
    for p in pttrns {
        println!("{}", p);
    }
}
fn number_to_pattern() {
    let number = readline().parse::<usize>().unwrap();
    let k = readline().parse::<usize>().unwrap();
    let pttrn = patterns::number_to_pattern(number,k);
    println!("{:?}", pttrn);
}
pub fn computing_frequences() {
    let text = readline();
    let k = readline().parse::<usize>().unwrap();
    let frequences = patterns::computing_frequences(&text, k);
    println!("{:?}", frequences);
} 
fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 || args.len() > 2 {
        println!("this version requires exactly one command line argument.");
        return;
    }
    match &args[1] as &str {
        "-pc" => pattern_count(),
        "-ptn" => pattern_to_number(),
        "-ntp" => number_to_pattern(),
        "-fw" => most_frequent_words(),
        "-fwm" => find_frequent_word_with_missmatch(),
        "-fwms" => find_frequent_word_with_missmatch_by_sorting(),
        "-fwmrs" => find_frequent_word_with_missmatch_and_reverse_by_sorting(),
        "-rc" => reverse_complement(),
        "-pm" => pattern_matching(),
        "-apm" => approximate_pattern_matching(),
        "-clf" => clump_finding(), 
        "-ms" => minimum_skew(),
        "-hd" => hamming_distance(),
        "-cf" => computing_frequences(),
        "-n" => neighbors(),
        &_ => println!("command {} not recognized.",&args[1])
    }
}