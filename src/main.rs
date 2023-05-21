use std::{env::args, process::exit, io};
// use atty::Stream;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("検索文字列と検索対象の入力がありません。");
        exit(0);
    } else if args.len() < 3 {
        // let stdout = io::stdout();
        // if stdout.isatty() {
        //     println!("exists!");
        // } else {
            println!("検索対象の入力がありません。");
            exit(0);
        // }
    }
    let pattern = args[1].to_string();
    let text = args[2].to_string();
    for line in text.lines() {
        if line.contains(&pattern) {
            println!("{line}");
        } else {
            println!("Not found: {line}");
        }
    }
}
