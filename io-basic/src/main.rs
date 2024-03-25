use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
extern crate clap;
use clap::{App, Arg};

fn main() -> Result<(), Error> {
    let matches = App::new("MayApp")
        .version("0.1")
        .author("kayryu")
        .about("Learn use Rust Crate!")
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("verbosity level"),
        )
        .args_from_usage("-p, --path=[FILE] 'Target file you want to change'")
        .get_matches();

    let path = "lines.txt";

    // 创建一个文件
    let mut output = File::create(path)?;
    write!(output, "Rust\n💖\nFun")?;

    // 打开文件
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}
