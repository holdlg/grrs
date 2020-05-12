use std::io;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        // do_hard_work();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let pattern = std::env::args().nth(1).expect("no pattern given");
//     let path = std::env::args().nth(2).expect("no path given");
//     println!("pattern: {}", pattern);
//     println!("path: {}", path);
//     let content = std::fs::read_to_string(&path)?;
//     println!("file content: {}", content);
//     Ok(())
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let pattern = std::env::args().nth(1).expect("no pattern given");
//     let path = std::env::args().nth(2).expect("no path given");
//     println!("pattern: {}", pattern);
//     println!("path: {}", path);
//     let result = std::fs::read_to_string(&path);
//     let content = match result {
//         Ok(content) => { content },
//         Err(error) => { return Err(error.into()); }
//     };
//     println!("file content: {}", content);
//     Ok(())
// }



// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let abc2 = String::from("xxx");
//     let mut abc = String::from("xxx");
//     println!("Hello, world!");
//     io::stdin()
//         .read_line(&mut abc)
//         .expect("Failed to read line");
//     println!("{}", abc);
//     println!("{}", abc.trim() == abc2);
//     let pattern = std::env::args().nth(1).expect("no pattern given");
//     let path = std::env::args().nth(2).expect("no path given");
//     println!("pattern: {}", pattern);
//     println!("path: {}", path);

//     let args = Cli::from_args();
//     // let content = std::fs::read_to_string(&args.path).expect("could not read file");
//     // for line in content.lines() {
//     //     if line.contains(&args.pattern) {
//     //         println!("{}", line)
//     //     }
//     // }

//     // let result = std::fs::read_to_string(&args.path);
//     // match result {
//     //     Ok(content) => { println!("File content: {}", content); }
//     //     Err(error) => { println!("Oh noes: {}", error); }
//     // }

//     // let result = std::fs::read_to_string(&args.path);
//     // let content = match result {
//     //     Ok(content) => { content },
//     //     Err(error) => { panic!("Can't deal with {}, just exit here", error); }
//     // };
//     // println!("file content222: {}", content);

//     // let content = std::fs::read_to_string(&args.path).unwrap();
//     // println!("xxxx {}", content);

//     let content = std::fs::read_to_string(&args.path);
//     let result = match content {
//         Ok(content) => content,
//         Err(error) => {
//             return Err(error.into());
//         }
//     };
//     println!("file content: {}", result);
//     Ok(())
// }
