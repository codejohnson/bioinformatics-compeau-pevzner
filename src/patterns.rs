    //--------------------------------------------
    //challenge 1A, Bioinformatics Compeau-Pevzner
    //
    //PATTERN COUNT
    //Se cuenta cuántas veces está el patrón pttrn en la cadena text
    //
    //--------------------------------------------
    pub fn pattern_count(text:&str,pttrn:&String) -> usize {
        if text.len() < pttrn.len() {return 0}
        let mut c:usize = 0;
        for i in 0..=text.len() - pttrn.len() {
            c += (&text[i..i+pttrn.len()] == pttrn) as usize;
        }
        c
    }


    //--------------------------------------------
    //challenge 1B, Bioinformatics Compeau-Pevzner
    //
    //FREQUENT WORDS
    //Se determina el k-mer de tamaño k más frecuente en el texto text
    //
    //--------------------------------------------
    use std::collections::HashSet;
    pub fn most_frequent_words(text:&str,k:usize) -> (HashSet<String>,usize) {
        let mut frequent_patterns = HashSet::<String>::new();
        let n = text.len() - k + 1;
        let mut count = vec![0;n];
        for i in 0..n {
            count[i] = pattern_count(&text, &text[i..i+k].to_string());
        }
        let max_count = count.iter().max().unwrap();
        for i in 0..n {
            if count[i] == *max_count {
                frequent_patterns.insert(text[i..i+k].to_string());
            }
        }
        (frequent_patterns,*max_count)
    }

    //--------------------------------------------
    //challenge 1C, Bioinformatics Compeau-Pevzner
    //
    //REVERSE COMPLEMENT
    //Find the reverse complement of a DNA string
    //
    //--------------------------------------------
    use std::collections::HashMap;
    pub fn reverse_complement(text:String) -> String {
        let mut nuc_comp:HashMap<char,char> = HashMap::new();
        nuc_comp.insert('A','T');
        nuc_comp.insert('C','G');
        nuc_comp.insert('G','C');
        nuc_comp.insert('T','A');
        let mut rev = String::new();
        for c in text.chars().rev() {
            rev.push(*nuc_comp.get(&c).unwrap());
        }
        rev
    }

    //--------------------------------------------
    //challenge 1D, Bioinformatics Compeau-Pevzner
    //
    //PATTERN MATCHING
    //Find positions of all ocurrences of a pattern in a string
    //
    //--------------------------------------------
    pub fn pattern_matching(pttrn:&str, text:String) -> Vec<usize> {
        let mut positions:Vec<usize> = Vec::new();
        if text.len() < pttrn.len() {return positions }
        for i in 0..=text.len() - pttrn.len() {
            if &text[i..i+pttrn.len()] == pttrn {
                positions.push(i);
            }
        }
        positions
    }

    //--------------------------------------------
    //challenge 1F, Bioinformatics Compeau-Pevzner
    //
    //MINIMUM SKEW
    //Find A position in a genome where a skew diagram attains a minimum
    //
    //--------------------------------------------

    pub fn minimum_skew(text:&String) -> std::vec::Vec<usize> {
        struct Skews {value:i32, loci:Vec::<usize>};
        let mut minpos = Skews{value:i32::MAX,loci:vec![0]};
        let mut skew:i32 = 0;
        let seq = text.as_bytes();
        for i in 0..seq.len() {
            if seq[i] == b'C' { skew -= 1}
            else if seq[i] == b'G' { skew += 1}
            if skew < minpos.value {
                minpos.value = skew;
                minpos.loci.clear();
                minpos.loci.push(i+1);
            }
            else if skew == minpos.value {
                minpos.loci.push(i+1);
            }
        }
        minpos.loci
    }

    //--------------------------------------------
    //challenge 1G, Bioinformatics Compeau-Pevzner
    //
    // HAMMING DISTANCE
    // Compute the Hamming distance between two DNA strings.
    //
    //--------------------------------------------

    pub fn hamming_distance(s:&str,t:&str) -> u32 {
        let mut dist = 0;
        let ( sb, tb )= (s.as_bytes(), t.as_bytes());
        for i in 0..sb.len() {
            dist += (sb[i] != tb[i]) as u32;
        }
        dist
    }

    //--------------------------------------------
    //challenge 1H, Bioinformatics Compeau-Pevzner
    //
    //APPROXIMATE PATTERN MATCHING
    //Find all aprproximate occurrences of a pattern in a string, with mismatches
    //
    //--------------------------------------------
    pub fn approximate_pattern_matching(pttrn:&str, text:String, mismatch_number:u32) -> Vec<usize> {
        let mut positions:Vec<usize> = Vec::new();
        if text.len() < pttrn.len() {return positions }
        for i in 0..=text.len() - pttrn.len() {
            if  hamming_distance(&text[i..i+pttrn.len()],pttrn) <= mismatch_number {
                positions.push(i);
            }
        }
        positions
    }