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
    pub fn reverse_complement(text:&str) -> String {
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
    //challenge 1E, Bioinformatics Compeau-Pevzner
    //
    //CLUMP FINDING
    //Find patterns forming clumps in a string
    //This functions return All distinct patterns forming (L,t)-clups in text
    //Based in improved version on page 46
    //--------------------------------------------
    pub fn clump_finding(genome:&str, k:usize, l:usize, t:usize) -> HashSet<String> {
        let gnm:String = genome.to_string();
        let mut patterns:HashSet<String> = HashSet::new();
        let mut clump: Vec<u64> = vec![0; (4usize).pow(k as u32)];
        let text = &genome[0..=l];
        let mut frequency_array = computing_frequences(text, k);
        for i in 0..(4usize).pow(k as u32)-1 {
            if frequency_array[i] >= t {
                clump[i] = 1;
            }
        }
        for i in 1..gnm.len() - l {
            let first_pattern = &genome[i-1..i-1+k];
            let mut index = pattern_to_number(first_pattern);
            frequency_array[index] -= 1;
            let last_pattern = &genome[i+l-k..i+l];
            index = pattern_to_number(last_pattern);
            frequency_array[index] += 1;
            if frequency_array[index] >= t {
                clump[index] = 1;
            }
        }
        for i in 0..(4usize).pow(k as u32) - 1 {
            if clump[i] == 1 {
                let pttrn = number_to_pattern(i,k);
                patterns.insert(pttrn);
            }
        }
        patterns
    }


    //--------------------------------------------
    //challenge 1F, Bioinformatics Compeau-Pevzner
    //
    //MINIMUM SKEW
    //Find A position in a genome where a skew diagram attains a minimum
    //
    //--------------------------------------------

    pub fn minimum_skew(text:&String) -> std::vec::Vec<usize> {
        struct Skews {
            value:i32, loci:Vec::<usize>
        }
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

    pub fn hamming_distance(s:&str,t:&str) -> usize {
        let mut dist = 0usize;
        let ( sb, tb )= (s.as_bytes(), t.as_bytes());
        for i in 0..sb.len() {
            dist += if sb[i] != tb[i] {1} else {0};
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
    pub fn approx_pattern_matching(pttrn:&str, text:String, mismatch_number:usize) -> Vec<usize> {
        let mut positions:Vec<usize> = Vec::new();
        if text.len() < pttrn.len() {return positions }
        for i in 0..=text.len() - pttrn.len() {
            if  hamming_distance(&text[i..i+pttrn.len()],pttrn) <= mismatch_number {
                positions.push(i);
            }
        }
        positions
    }

    //--------------------------------------------
    //challenge 1I, Bioinformatics Compeau-Pevzner
    //
    //FREQUENT WORDS WITH MISSMATCH
    //Se determina el k-mer de tamaño k más frecuente en el texto text
    //
    //--------------------------------------------
    pub fn freq_word_miss_sort(text:&str,k:usize, d:usize) -> HashSet<String> {
        let mut frequent = HashSet::<String>::new();
        let mut neigborhoods = Vec::<String>::new();
        for i in 0..=text.len()-k {
            let ngbhrs = neighbors(&text[i..i+k], d);
            for nb in ngbhrs {
                neigborhoods.push(nb);
            }
        }
        let mut index = vec![0;neigborhoods.len()];
        let mut count = vec![1;neigborhoods.len()];
        for i in 0..neigborhoods.len() {
            index[i] = pattern_to_number(&neigborhoods[i]);
        }
        index.sort();
        for i in 0..neigborhoods.len() - 1 {
            if index[i] == index[i+1] {
                count[i+1] = count[i] + 1;
            }
        }
        let max_count:usize = *count.iter().max().unwrap();
        for i in 0..neigborhoods.len()  {
            if count[i] == max_count {
                let pttrn = number_to_pattern(index[i], k);
                frequent.insert(pttrn);
            }
        }
        frequent
    }

    //--------------------------------------------
    //challenge 1I, Bioinformatics Compeau-Pevzner
    //
    //FREQUENT WORDS WITH MISSMATCH
    //Se determina el k-mer de tamaño k más frecuente en el texto text
    //
    //--------------------------------------------
    pub fn freq_word_miss_rev_sort(text:&str,k:usize, d:usize) -> HashSet<String> {
        let mut frequent = HashSet::<String>::new();
        let mut neigborhoods = Vec::<String>::new();
        for i in 0..=text.len()-k {
            let ngbhrs = neighbors(&text[i..i+k], d);
            let rcomp = reverse_complement(&text[i..i+k]);
            let rngbhrs = neighbors(&rcomp, d);
            for nb in ngbhrs {
                neigborhoods.push(nb);
            }
            for nb in rngbhrs {
                neigborhoods.push(nb);
            }
        }
        let mut index = vec![0;neigborhoods.len()];
        let mut count = vec![1;neigborhoods.len()];
        for i in 0..neigborhoods.len() {
            index[i] = pattern_to_number(&neigborhoods[i]);
        }
        index.sort();
        for i in 0..neigborhoods.len() - 1 {
            if index[i] == index[i+1] {
                count[i+1] = count[i] + 1;
            }
        }
        let max_count:usize = *count.iter().max().unwrap();
        for i in 0..neigborhoods.len()  {
            if count[i] == max_count {
                let pttrn = number_to_pattern(index[i], k);
                frequent.insert(pttrn);
            }
        }
        frequent
    }
    //--------------------------------------------
    //challenge 1K, Bioinformatics Compeau-Pevzner
    //
    //COMPUTE FREQUENCIES
    // generar un arreglo de frecuencias para los k-mers de una secuencia
    //
    //--------------------------------------------
    pub fn computing_frequences(text:&str,k:usize) -> Vec<usize> {
        let mut frequency_array: Vec<usize> = vec![0; (4usize).pow(k as u32)];
        for i in 0..=text.len()-k as usize {
            let pttrn = &text[i..i+k];
            let j = pattern_to_number(pttrn) as usize;
            frequency_array[j] += 1;
        }
        frequency_array
    }  

    //--------------------------------------------
    //challenge 1L, Bioinformatics Compeau-Pevzner
    //
    //PATTERN TO NUMBER
    //Se convierte un patrón de secuencia DNA a un número entero
    //
    //--------------------------------------------
    fn sym_to_num(c:char)->usize { "ACGT".find(c).unwrap()}

    fn prefix<'a>(p: &'a str) -> &'a str { 
        if p.len() == 1 {
            return &""
        } 
        return &p[0..p.len()-1]; 
    }
    fn last_symbol<'a>(p: &'a str) -> char  {p.chars().last().unwrap() }

    pub fn pattern_to_number(pattern:&str)->usize {
        if pattern.len() == 0 {return 0}
        let pre = prefix(&pattern);
        let sym = last_symbol(&pattern);
        4 * pattern_to_number(pre) + sym_to_num(sym)
    }

    //--------------------------------------------
    //challenge 1M, Bioinformatics Compeau-Pevzner
    //
    //NUMBER TO PATTERN
    //Se convierte un número entero a un patrón de secuencia DNA
    //
    //--------------------------------------------
    fn num_to_sym(n:usize)-> String { (['A','C','G','T'][n as usize]).to_string()}

    pub fn number_to_pattern(index:usize,k:usize)->String {
        if k == 1 {return num_to_sym(index)}
        let q: usize = index / 4;
        let r = index % 4;
        let sym = num_to_sym(r);
        let mut prefix_pattern = number_to_pattern(q, k - 1);
        prefix_pattern.push_str(&sym);
        prefix_pattern
    }

    //--------------------------------------------
    //challenge 1N, Bioinformatics Compeau-Pevzner
    //
    //NEIGHBORS  
    //The d-neighborhood Neighbors(Pattern, d) is the set of all k-mers 
    //whose Hamming distance from Pattern does not exceed d.
    //
    //--------------------------------------------
    pub fn neighbors<'a>(pattern:&str, d:usize) -> Vec<String> {
        if d == 0 { 
            let mut alone = Vec::<String>::new();
            alone.push(String::from(pattern));
            return alone
        }
        if pattern.len() == 1 {
            let mut nucleotides = Vec::<String>::new();
            nucleotides.push("A".to_string());
            nucleotides.push("C".to_string());
            nucleotides.push("G".to_string());
            nucleotides.push("T".to_string());
            return nucleotides
        }
        let mut neighborhood = Vec::<String>::new();
        let suffix_neighbors = neighbors(&pattern[1..], d);
        for text in suffix_neighbors {
            if hamming_distance(&pattern[1..], &text) < d {
                for &n in ["A","C","G","T"].iter() {
                    let mut neigborh:String = String::new();
                    neigborh.push_str(n);
                    neigborh.push_str(&text);
                    neighborhood.push(neigborh);
                }
            }
            else {
                let mut neigborh:String = String::from(&pattern[..=0]);
                neigborh.push_str(&text);
                neighborhood.push(neigborh);
            }
        }
        neighborhood
    }