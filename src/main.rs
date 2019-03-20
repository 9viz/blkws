use std::{env, process, path::Path};
use blkws;

fn usage() {
    println!("Usage: blkws [block/unblock] website");
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        eprintln!("error: not enough arguments!");
        usage();
    }

    if vec!["help", "-h", "--help"].contains(&args[1].as_str()) {
        usage();
    }

    let rc = env::var("BLKWS_RC")
        .unwrap_or_else( |_| {
            let c = env::var("XDG_CONFIG_HOME")
                .unwrap_or_else(|_| {
                    format!("{}/.config", env::var("HOME").unwrap())
                });

            format!("{}/blkwsrc", c)
        });

    if ! Path::new(&rc).is_file() {
        eprintln!("error: blkwsrc does not exist");
        process::exit(1);
    }

    match &args[1].as_str() {
        &"block" => { blkws::block(&rc, &args[2].as_str()); },
        &"unblock" => { blkws::unblock(&rc, &args[2].as_str()); },
        _ => { eprintln!("unrecongnized flag!"); usage(); },
    }

    process::exit(0);
}
