mod test;

use ansi_term::Color;
use clap::Parser;
use std::{collections::HashMap, fs::File, io::Error, io::Read};

#[derive(Parser, Debug)]
struct Args {
    #[arg(index = 1)]
    path: String,
    #[arg(short, long, default_value = "false")]
    desc: bool,
    #[arg(short, long, default_value = "false")]
    asc: bool,
    #[arg(short, long, default_value = "false")]
    ignore_case: bool,
    #[arg(short = 'n', long = "lines")]
    limit: Option<usize>,
}

fn word_count(contents: &str) -> HashMap<&str, usize> {
    let mut counter = HashMap::new();
    contents.split_whitespace().for_each(|word| {
        let count = counter.entry(word).or_insert(0);
        *count += 1;
    });
    counter
}

fn run(args: Args) -> Result<(), Error> {
    match File::options().read(true).open(args.path) {
        Err(e) => eprintln!("Error opening file: {:?}", e),
        Ok(mut file) => {
            // 读取文件内容
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            if args.ignore_case {
                contents = contents.to_lowercase();
            }

            let counter = word_count(&contents);
            let mut counter: Vec<_> = counter.into_iter().collect();

            if args.desc {
                // 降序
                counter.sort_by(|a, b| b.1.cmp(&a.1));
            } else if args.asc {
                // 升序
                counter.sort_by(|a, b| a.1.cmp(&b.1));
            }

            match args.limit {
                Some(limit) => counter.iter().take(limit),
                None => counter.iter().take(counter.len()),
            }
            .for_each(|(word, count)| {
                println!("{} : {}", Color::Green.paint(*word), count);
            });
        }
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    if let Err(err) = run(args) {
        eprintln!("Error: {:?}", err);
    }
}
