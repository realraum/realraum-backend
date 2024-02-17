use std::fs;

use std::path::PathBuf;

use crate::data;

/// Lists all sounds in the [`BASE_PATH`] directory, returning a [`Vec`] of [`Sound`] structs.
pub(crate) fn index_sounds_from_disk(base_path: &PathBuf) -> Vec<data::Sound> {
    let mut sounds = Vec::new();
    println!("Searching for sounds in {}", base_path.display());
    for entry in fs::read_dir(&*base_path).unwrap() {
        if let Ok(entry) = entry {
            if let Some(filename) = entry.file_name().to_str() {
                let filepath = entry.path();
                let fname = filepath.strip_prefix(&*base_path).unwrap_or(&filepath);
                let fname = fname.to_str().unwrap_or("");
                let Ok(file_contents_bin) = fs::read(&filepath) else {
                    // TODO handle subdirectories
                    println!("Failed to read file {}", filename);
                    continue;
                };
                let md5sum: [u8; 16] = md5::compute(&file_contents_bin).0;
                sounds.push(data::Sound {
                    name: filename.to_string(),
                    path: fname.to_string(),
                    md5sum,
                    id: 0,
                    play_count: 0,
                });
            }
        }
    }
    sounds
}
