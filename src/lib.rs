use std::{fs, process, io::Write};
pub mod rand;
pub mod util;

pub fn block(links_path: &str, website: &str) {
    /*
     * write to /etc/hosts
     */
    let links = util::get_links(links_path);
    let links = util::grab_websitelinks(links.as_str(), website);
    let block_string = util::mkblockstring(&links);

    let host_file: &str = "/etc/hosts";

    let mut fhosts = fs::OpenOptions::new()
        .append(true)
        .open(host_file)
        .unwrap_or_else(|e| {
            eprintln!("error: {}", e);
            process::exit(1);
        });

    fhosts.write_all(block_string.as_str().as_bytes())
        .unwrap_or_else(|e| {
            eprintln!("error {}", e);
            process::exit(1);
        });
}

pub fn unblock(links_path: &str, website: &str) {
    /*
     * unblock a website
     */
    if ! util::puzzle() {
        println!("You didn't win the puzzle! Can't unblock {}", website);
        process::exit(1);
    }

    let links = util::get_links(links_path);

    let blocked_websites: Vec<&str> = util::grab_websitelinks(
        links.as_str(),
        website
    );

    let mut new_hostfile: String = String::new();

    let cur_hostfile = fs::read_to_string(&"/etc/hosts")
        .unwrap_or_else(|e| {
            eprintln!("error: {}", e);
            process::exit(1);
        });

    let cur_hostfile: Vec<&str> = cur_hostfile.lines()
        .collect();

    for line in cur_hostfile.iter() {
        if ! blocked_websites.contains(&line) {
            new_hostfile.push_str(line);
        }
    }

    fs::write(&"/etc/hosts", new_hostfile.as_str())
        .unwrap_or_else(|e| {
            eprintln!("error: {}", e);
            process::exit(1);
        });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unblock_test() {
        unblock(&"/home/viz/usr/src/rust/blkws/src/template", &"website");
    }
}
