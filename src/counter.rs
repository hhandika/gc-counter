pub mod fasta {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    pub fn read_file(fname: &String) -> Vec<String> {
        let f = File::open(fname).unwrap();
        let buff = BufReader::new(f);
        buff.lines()
            .map(|l| l.expect("Failed!"))
            .collect()
    }

    fn calculate_gc_ratio(dna: &String) -> f64 {
        let n = dna.len() as f64;
        count_gc_content(dna) as f64 / n
    }
    
    fn count_gc_content(dna: &String) -> u32 {
        let mut gc_content: u32 = 0;
        for i in dna.chars() {
            match i {
                'G' | 'g'  => gc_content += 1,
                'C' | 'c' => gc_content += 1,
                _ => (), 
            };
        }
        gc_content
    }

    pub fn print_results(seq: &Vec<String>) {
        for line in seq {
            if line.starts_with(">") {
                print!("{}: ", line.replace(">", "").replace("_", " "));
            } else {
                println!("{:.4}%", calculate_gc_ratio(line));
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn gc_content_test() {
            let a: String = String::from("AA");
            let b: String = String::from("AAGC");
            let c: String = String::from("aaAA");
            let d: String = String::from("aattggcc");
            assert_eq!(0, count_gc_content(&a));
            assert_eq!(2, count_gc_content(&b));
            assert_eq!(0, count_gc_content(&c));
            assert_eq!(4, count_gc_content(&d));
        }

        #[test]
        fn gc_ratio_test() {
            let a: String = String::from("AA");
            let b: String = String::from("AAGC");
            let c: String = String::from("aaAA");
            let d: String = String::from("aattggcc");
            assert_eq!(0.0, calculate_gc_ratio(&a));
            assert_eq!(0.5, calculate_gc_ratio(&b));
            assert_eq!(0.0, calculate_gc_ratio(&c));
            assert_eq!(0.5, calculate_gc_ratio(&d));
        }
    }
}