use std::fs::{self, ReadDir};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let search_dir: &str = if args.len() > 1 {
        args[1].as_str()
    } else {
        "."
    };

    let dry_run: bool = args.contains(&"--dry-run".to_string());

    println!("Searching .zoneIdentifier files in: {}", search_dir);

    if dry_run {
        println!("Dry run on");
    }

    let mut deleted: u32 = 0;
    let mut found: u32 = 0;

    scan_dir(Path::new(search_dir), &mut deleted, &mut found, dry_run);

    println!("\nFound: {} | Deleted: {}", found, deleted);
}

fn scan_dir(dir: &Path, deleted: &mut u32, found: &mut u32, dry_run: bool) {
    let files: ReadDir = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for file in files.flatten() {
        let path = file.path();

        if path.is_dir() {
            scan_dir(&path, deleted, found, dry_run)
        } else {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

            if name.ends_with(".ZoneIdentifier") || name.ends_with(":Zone.Identifier") {
                *found += 1;
                println!("🗑️  {}", path.display());

                if !dry_run {
                    match fs::remove_file(&path) {
                        Ok(_) => *deleted += 1,
                        Err(e) => println!("   ❌ Błąd: {}", e),
                    }
                }
            }
        }
    }
}
