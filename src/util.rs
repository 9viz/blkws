use std::{fs, process, io::{stdin, stdout, Write}};
use crate::rand;

pub fn get_links(links_path: &str) -> String {
    let links = fs::read_to_string(links_path)
        .unwrap_or_else(|e| {
            println!("error: {}", e);
            process::exit(0);
        });

    links
}

pub fn grab_websitelinks<'a>(links: &'a str, website: &'a str) -> Vec<&'a str> {
    /*
     * links is the entire content of the links file
     *
     * grab all the links associated with website
     */

    let links: Vec<&str> = links.lines().collect();

    let ind_website = links.iter()
        .position(|&x| x == format!("[{}]", website))
        .unwrap_or_else(|| {
            eprintln!("failed to parse the config");
            process::exit(1);
        });

    let ind_end = links.iter()
        .position(|&x| x == format!("[/{}]", website))
        .unwrap_or_else(|| {
            eprintln!("failed to parse the config");
            process::exit(1);
        });

    return links[ind_website+1..ind_end].to_vec();
}

pub fn mkblockstring(website_links: &Vec<&str>) -> String {
    /*
     * block websitelinks in /etc/hosts
     */

    let mut content_hosts = String::new();

    for l in website_links.iter() {
        content_hosts.push_str(
            &format!("0.0.0.0\t{}\n", l)
        );
    }

    return content_hosts;
}

pub fn puzzle() -> bool {
    let input = |prompt: &str| -> u64 {
        let mut inp = String::new();

        print!("{}", prompt);
        stdout()
            .flush()
            .unwrap();

        stdin()
            .read_line(&mut inp)
            .expect("can't read line");

        let inp: u64 = inp.trim()
            .parse()
            .unwrap_or_else(|_| {0});

        inp
    };

    let mut won: bool = true;

    for _ in 0..5 {
        unsafe {
        let n1 = rand::rand_range(7, 13);
        let n2 = rand::rand_range(13, 25);
        let r = n1 + n2;

        let uans = input(&format!("What is {} + {}? ", n1, n2));

        if r != uans {
            won = false;
            break;
        }
        }
    }

    won
}
