use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead, Write};

fn main() {
    // Read content IDs from scraped_content_ids.txt into a HashSet
    let mut content_ids: HashSet<String> = HashSet::new();
    let content_file = File::open("scraped_content_ids.txt").expect("Error opening content file");
    let content_reader = BufReader::new(content_file);

    for line in content_reader.lines() {
        let id = line.expect("Error reading content file line");
        content_ids.insert(id.trim().to_string());
    }

    // Read link IDs from scraped_links_ids.txt
    let link_file = File::open("scraped_links_ids.txt").expect("Error opening link file");
    let link_reader = BufReader::new(link_file);

    // Open output file for writing
    let mut output_file = File::create("output.txt").expect("Error creating output file");

    // Find missing IDs and write them to output.txt
    for line in link_reader.lines() {
        let id = line.expect("Error reading link file line");
        let trimmed_id = id.trim().to_string();
        if !content_ids.contains(&trimmed_id) {
            writeln!(output_file, "{}", trimmed_id).expect("Error writing to output file");
        }
    }

    println!("Missing IDs written to output.txt");
}
