use std::error::Error;
use std::fs::File;
use std::path::Path;
use tar::Archive;

pub async fn decompress(path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let mut archive: Archive<File> = Archive::new(file);

    let path = Path::new(path);
    let unpack_dir = path
        .parent()
        .unwrap_or_else(|| Path::new("unpack_directory"));
    archive.unpack(unpack_dir)?;
    println!("TAR file extracted to {}", unpack_dir.display());
    Ok(())
}
