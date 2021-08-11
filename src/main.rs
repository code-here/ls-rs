use chrono::{DateTime, Utc};
use std::{env, error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = env::args().skip(1).collect::<String>();
    if path.is_empty() {
        path = ".".to_owned()
    };
    let path = PathBuf::from(&path);
    if path.is_dir() {
        let paths = fs::read_dir(path)?;
        for path in paths {
            let path = path?.path();
            let (path, modified, size) = get_file_data(path)?;
            println!(
                "{:>5}  {:>5}  {}",
                size,
                modified.format("%d %b  %H:%M"),
                path.as_path().display()
            );
        }
    } else {
        let (path, modified, size) = get_file_data(path)?;
        println!(
            "{:>5}  {:>5}  {}",
            size,
            modified.format("%d %b  %H:%M"),
            path.display()
        );
    }
    Ok(())
}

fn get_file_data(path: PathBuf) -> Result<(PathBuf, DateTime<Utc>, String), Box<dyn Error>> {
    let modified: DateTime<Utc> = path.metadata()?.modified()?.into();
    let size = path.metadata()?.len();
    let mut mod_size = format!("{}B", size);
    if size > 1000 {
        mod_size = format!("{:.1}KB", size as f64 / 1000.0);
    }
    Ok((path, modified, mod_size))
}
