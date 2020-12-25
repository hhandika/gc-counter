pub mod fasta {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::io::LineWriter;
    use std::process;

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

    fn check_fasta(input: &String) -> bool {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        let mut result = false;
        for line_ in reader.lines() {
            let line = line_.unwrap();
            if line.starts_with(">") {
                result = true;
            } 
        }
        result
    }

    // Parse fasta file
    // Get gc content and ratio
    // Write the results to csv.
    pub fn parse_fasta(input: &String, output: &String) {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);

        let output_file = format!("{}{}", output, ".csv");
        let file = File::create(&output_file).unwrap();
        let mut file = LineWriter::new(file);

        let is_fasta = check_fasta(&input);
        if !is_fasta {
            println!("Invalid fasta file");
            process::abort()
        }

        writeln!(file, "Id,GC-Content, GC-Ratio").unwrap();
        for line_ in reader.lines() {
            let line = line_.unwrap();
            if line.starts_with(">") {
                let id = line.replace(">", "").replace("_", " ");
                write!(file, "{},", &id).unwrap();
            } else {
                let gc_content = count_gc_content(&line);  
                let gc_ratio = calculate_gc_ratio(&line);
                writeln!(file, "{},{:.4}", &gc_content, &gc_ratio).unwrap();
            }
            
        }
        println!("Done counting! The result is saved as {}", &output_file);
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

        #[test]
        fn check_fasta_test() {
            let fasta = String::from("test_files/COIII.fasta");
            let not_fasta = String::from("test_files/COIII.nex");
            assert_eq!(true, check_fasta(&fasta));
            assert_eq!(false, check_fasta(&not_fasta));
        }
    }
}