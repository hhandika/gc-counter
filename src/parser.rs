pub parse {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    pub fn parse_fastaq(input: &String; output: &String) {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);

        let output_file = format!("{}{}", output, ".csv");
        let file = File::create(&output_file).unwrap();
        let mut file = LineWriter::new(file);

        // writeln!(file, "Read,GC-Content, GC-Ratio").unwrap();
        let mut read: i32 = 0;
        for l_ in reader.lines() {
            let line = l_.unwrap();
            if line.starts_with("@") {
                read += 1
                print!("Read {}", read);
            } 
            while !line.starts_with("+") {
                let seq = line;
                println!("{}", seq);
            }
        }
    }
}