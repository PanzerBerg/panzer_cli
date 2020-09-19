use std::io::{BufReader, BufRead};
use std::path;
use std::fs::File;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let content = File::open(&args.path).expect("Cannot open file");
    let reader = BufReader::new(content);    

    let mut has_pattern = false;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(&args.pattern) {
            has_pattern = true;
            println!("{}", line);
        }
    }

    if !has_pattern {
        println!("No pattern {} found in {}", &args.pattern, &args.path.display().to_string());
    }
}
