use std::fs::{self, ReadDir};
use std::path::Path;

struct Config {
    search_dir: String,
    recursive: bool,
    dry_run: bool,
}

fn parse_args() -> Config {
    let args: Vec<String> = std::env::args().collect();

    let recursive = args.contains(&"-r".to_string()) || args.contains(&"-R".to_string());
    let dry_run = args.contains(&"--dry-run".to_string());

    let search_dir = args.iter().skip(1)
        .find(|a| !a.starts_with('-'))
        .cloned()
        .unwrap_or_else(|| ".".to_string());

    Config { search_dir, recursive, dry_run }
}

fn main() {
    let config = parse_args();

    println!("Searching .ZoneIdentifier files in: {}", config.search_dir);

    if config.dry_run {
        println!("Dry run on");
    }

    let mut deleted: u32 = 0;
    let mut found: u32 = 0;

    scan_dir(Path::new(&config.search_dir), &config, &mut deleted, &mut found);

    println!("\nFound: {} | Deleted: {}", found, deleted);
}

fn scan_dir(dir: &Path, config: &Config, deleted: &mut u32, found: &mut u32) {
    let files: ReadDir = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for file in files.flatten() {
        let path = file.path();

        if path.is_dir() {
            if config.recursive {
                scan_dir(&path, config, deleted, found);
            }
            continue;
        }

        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        check_zone_identifier(name, &path, config, deleted, found);
    }
}

fn check_zone_identifier(name: &str, path: &Path, config: &Config, deleted: &mut u32, found: &mut u32) {
    if name.ends_with(".ZoneIdentifier") {
        *found += 1;
        println!("{}", path.display());

        if !config.dry_run {
            match fs::remove_file(path) {
                Ok(_) => *deleted += 1,
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}
