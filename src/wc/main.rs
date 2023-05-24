use clap::Parser;
use std::{collections::HashMap, fs::File, io::Read};

#[derive(Parser, Debug)]
struct Args {
    #[arg(index = 1)]
    path: String,
    #[arg(short, long, default_value = "false")]
    desc: bool,
    #[arg(short, long, default_value = "false")]
    asc: bool,
}

fn main() {
    let args = Args::parse();

    match File::options().read(true).open(args.path) {
        Err(e) => eprintln!("Error opening file: {:?}", e),
        Ok(mut file) => {
            // 读取文件内容
            let mut contents = String::new();
            if let Err(e) = file.read_to_string(&mut contents) {
                eprintln!("Error reading file: {:?}", e);
            }
            // Word count
            let counter = contents.lines().fold(HashMap::new(), |mut acc, line| {
                line.split_whitespace().for_each(|word| {
                    let count = acc.entry(word).or_insert(0);
                    *count += 1;
                });
                return acc;
            });

            if args.desc {
                // 降序
                let mut counter: Vec<_> = counter.into_iter().collect();
                counter.sort_by(|a, b| b.1.cmp(&a.1));
                counter.iter().for_each(|(word, count)| {
                    println!("{}: {}", word, count);
                });
                return;
            } else if args.asc {
                // 升序
                let mut counter: Vec<_> = counter.into_iter().collect();
                counter.sort_by(|a, b| a.1.cmp(&b.1));
                counter.iter().for_each(|(word, count)| {
                    println!("{}: {}", word, count);
                });
                return;
            } else {
                counter.iter().for_each(|(word, count)| {
                    println!("{}: {}", word, count);
                });
            }
        }
    }
}
