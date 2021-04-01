//------------------------------------------------
//  command line syntax:
//  dnaabox -flag < data_file_test_path
/*
    possible flags are:

    -pc     => pattern_count()              ......CHALLENGE 1A
    -fw     => most_frequent_words()        ......CHALLENGE 1B
    -rc     => reverse_complement()         ......CHALLENGE 1C
    -pm     => pattern_matching()           ......CHALLENGE 1D
    -clf    => clump_finding()              ......CHALLENGE 1E
    -ms     => minimum_skew()               ......CHALLENGE 1F
    -hd     => hamming_distance()           ......CHALLENGE 1G
    -apm    => approx_pattern_matching()    ......CHALLENGE 1H
    -fm     => freq_word_miss()             ......CHALLENGE 1I
    -fmr    => freq_word_miss_rev()         ......CHALLENGE 1J
    -cf     => computing_frequences()       ......CHALLENGE 1K
    -ptn    => pattern_to_number()          ......CHALLENGE 1L
    -ntp    => number_to_pattern()          ......CHALLENGE 1M
    -n      => neighbors()                  ......CHALLENGE 1N
*/
//------------------------------------------------

mod dnabox;
use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 || args.len() > 2 {
        println!("this version requires exactly one command line argument.");
        return;
    }
    match &args[1] as &str {
        "-pc" => dnabox::pattern_count(),
        "-ptn" => dnabox::pattern_to_number(),
        "-ntp" => dnabox::number_to_pattern(),
        "-fw" => dnabox::most_frequent_words(),
        "-fwm" => dnabox::freq_word_miss(),
        "-fm" => dnabox::freq_word_miss(),
        "-fmr" => dnabox::freq_word_miss_rev(),
        "-rc" => dnabox::reverse_complement(),
        "-pm" => dnabox::pattern_matching(),
        "-apm" => dnabox::approx_pattern_matching(),
        "-clf" => dnabox::clump_finding(), 
        "-ms" => dnabox::minimum_skew(),
        "-hd" => dnabox::hamming_distance(),
        "-cf" => dnabox::computing_frequences(),
        "-n" => dnabox::neighbors(),
        &_ => println!("command {} not recognized.",&args[1])
    }
}