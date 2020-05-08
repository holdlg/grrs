
use structopt::StructOpt;
use std::io;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let abc2 = String::from("xxx");
    let mut abc = String::from("xxx");
    println!("Hello, world!");
    println!("Hello, world!");
    io::stdin().read_line(&mut abc).expect("Failed to read line");
    println!("{}", abc);
    println!("{}", abc.trim() == abc2);
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    println!("{}", pattern);
    println!("{}", path);

    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line)
        }
    }
}
