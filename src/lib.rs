use std::{fs, process};

fn grab_websitelinks<'a>(links: &'a str, website: &'a str) -> Vec<&'a str> {
    /*
     * links is the entire content of the links file
     *
     * grab all the links associated with website
     */

    let links: Vec<&str> = links.lines().collect();
    let nlinks: usize = links.len() - 1;
    let mut site_links: Vec<&str> = vec![];

    let ind_website = links.iter()
        .position(|&x| x == format!("[{}]", website))
        .unwrap_or_else(|| {
            process::exit(1);
        });

    for l in links[ind_website+1..nlinks].iter() {
        if l == &"" { break; }
        site_links.push(l);
    }

    return site_links;
}

fn mkblockstring(website_links: &Vec<&str>) -> String {
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

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn grab_linktest() {
        let contents = fs::read_to_string(
            "/home/viz/usr/src/rust/blkhst/src/template"
        ).unwrap();

        println!("{:?}",
                 grab_websitelinks(&contents, "website")
                 );
    }

    #[test]
    fn block_test() {
        let contents = fs::read_to_string(
            "/home/viz/usr/src/rust/blkhst/src/template"
        ).unwrap();

        let links = grab_websitelinks(&contents, "website");

        println!("{}", mkblockstring(&links));
    }
}
