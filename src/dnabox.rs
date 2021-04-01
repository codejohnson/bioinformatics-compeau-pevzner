mod patterns;
use std::io;
use std::io::prelude::*;
fn readline()->String { io::stdin().lock().lines().next().unwrap().unwrap() }

//function to test 1A Challenge from command line
pub fn pattern_count() {
    let text = readline();
    let pattern = readline();
    println!("{}", patterns::pattern_count(&text, &pattern));
}

pub fn most_frequent_words() {
    let text = readline();
    let k = readline().parse::<usize>();
    if !k.is_err() {
        println!("{:?}", patterns::most_frequent_words(&text, k.unwrap()));
    }
}

pub fn freq_word_miss() {
    let text = readline();
    let values = readline();
    let split = values.split(' ').collect::<Vec<&str>>();
    let k = split[0].parse::<usize>().unwrap();
    let d = split[1].parse::<usize>().unwrap();
    println!("{:?}", patterns::freq_word_miss_sort(&text, k, d));
}

pub fn freq_word_miss_rev() {
    let text = readline();
    let values = readline();
    let split = values.split(' ').collect::<Vec<&str>>();
    let k = split[0].parse::<usize>().unwrap();
    let d = split[1].parse::<usize>().unwrap();
    println!("{:?}", patterns::freq_word_miss_rev_sort(&text, k, d));
}


pub fn reverse_complement() {
    let text = readline();
    println!("{:?}", patterns::reverse_complement(&text));
}

pub fn pattern_matching() {
    let text = readline();
    let pattern = readline();
    for pos in patterns::pattern_matching(&text,pattern).iter() {
        print!("{} ", pos);
    }    
    println!();
}

pub fn approx_pattern_matching() {
    let text = readline();
    let pattern = readline();
    let d = readline().parse::<usize>();
    for pos in patterns::approx_pattern_matching(&text,pattern,d.unwrap()).iter() {
        print!("{} ", pos);
    }
    println!();
}

pub fn clump_finding() {
    let text = readline();
    let arguments = readline();
    let values = arguments.split(" ").collect::<Vec<&str>>();
    let k_size =values[0].parse::<usize>().unwrap();
    let lwin =values[1].parse::<usize>().unwrap();
    let times =values[2].parse::<usize>().unwrap();
    let clump_patters = patterns::clump_finding(&text, k_size, lwin, times);
    println!("{:?}", clump_patters);
}

pub fn minimum_skew() {
    let text = readline();
    let minskews = patterns::minimum_skew(&text);
    println!("{:?}", minskews);
}

pub fn hamming_distance() {
    let s = readline();
    let t = readline();
    let distance = patterns::hamming_distance(&s, &t);
    println!("{:?}", distance);
}

pub fn pattern_to_number() {
    let pttrn = readline();
    let num = patterns::pattern_to_number(&pttrn);
    println!("{:?}", num);
}

pub fn neighbors() {
    let pttrn = readline();
    let d = readline().parse::<usize>().unwrap();
    let pttrns = patterns::neighbors(&pttrn,d);
    for p in pttrns {
        println!("{}", p);
    }
}

pub fn number_to_pattern() {
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
