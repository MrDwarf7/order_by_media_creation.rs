#![feature(windows_change_time, windows_by_handle)]
// mod error;
mod folders;
mod prelude;
mod processing;
mod seperators;
mod stamp_converter;

pub use processing::process_folder;

pub use crate::prelude::*;
pub use crate::seperators::Seperators;

pub static DATA_DIR: LazyLock<PathBuf> = LazyLock::new(data_dir);
pub const RUN_OPERATION: bool = false;

// TODO: Add a cli & add a proper logging system lol

// Apparently this can pull `media created at` from mp4 files
// https://github.com/alfg/mp4-rust

// There's also an ffmpeg crate lol, provides it as a hashmap and you can call .get("creation_time") on it

fn main() -> Result<()> {
    let crawl_type = CrawlType::Parallel;

    let start = std::time::Instant::now();
    println!("Starting...");

    let folders = folders::crawl_dir(&DATA_DIR.join("video_recur"), crawl_type)?;

    for folder in &folders.children {
        process_folder(folder);
    }

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    Ok(())
}

pub fn data_dir() -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    let data_dir = current_dir.join("data");
    data_dir
}

pub enum CrawlType {
    Serial,
    Parallel,
}

#[cfg(test)]
mod tests {

    use std::sync::LazyLock;

    use super::*;
    use crate::processing::get_creation_time;

    static T_DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| data_dir());
    const TEST_FILE: &str = "test.mp4";

    #[test]
    fn test_get_creation_time() {
        let test_file = T_DATA_DIR.join(TEST_FILE);
        let creation_time = get_creation_time(&test_file).unwrap();

        assert!(creation_time.contains("AM") || creation_time.contains("PM"));
    }

    #[test]
    fn test_data_dir() {
        let data_dir = data_dir();
        assert!(data_dir.exists());
        assert!(data_dir.is_dir());

        let test_file = data_dir.join(TEST_FILE);
        assert!(test_file.exists());
        assert!(test_file.is_file());
    }

    // #[ignore = "This test is not yet implemented"]
    // #[test]
    // fn test_main() {
    //     main().unwrap();
    // }
}
