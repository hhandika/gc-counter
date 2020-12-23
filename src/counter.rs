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
    
    fn count_gc_content(dna: &String) -> f64 {
        let mut gc_content: f64 = 0.0;
        for i in dna.chars() {
            if i == 'G' || i == 'C'  {
                gc_content += 1.0
            }
        }
        gc_content / dna.len() as f64
    }

    pub fn write_results(seq: &Vec<String>) {
        for line in seq {
            if line.starts_with(">") {
                print!("{}: ", line.replace(">", "").replace("_", " "));
            } else {
                println!("{:.4}%", count_gc_content(line));
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
            assert_eq!(0.0, count_gc_content(&a));
            assert_eq!(0.5, count_gc_content(&b))
        }
    }
}