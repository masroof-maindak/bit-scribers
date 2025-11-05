use std::env;
use std::error::Error;
use std::fs::File;
use std::process;

use bit_scribers::{scan_from_file, vec_to_int};

fn parse_args(args: &[String]) -> Result<(&String, &String), String> {
    if args.len() != 3 {
        let msg = format!(
            "Invalid argument count.\nUsage: {} <fpath> <byte offset>",
            args[0]
        );

        return Err(msg);
    }

    let fpath = &args[1];
    let offset_str = &args[2];

    Ok((fpath, offset_str))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let (fpath, offset_str) = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    let offset = offset_str
        .parse::<u64>()
        .expect("Offset should be an integer.");

    let f = File::open(fpath)?;

    let buf = scan_from_file(f, offset)?;

    let x = vec_to_int(buf);

    println!("{x:?}");

    Ok(())
}
