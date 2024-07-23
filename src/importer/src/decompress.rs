use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tar::Archive;
use zip::ZipArchive;

pub async fn decompress(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    //let file_path = determine_file_path(&url, &response).await?;
    //println!("File downloaded and saved as: {}", file_path.display());

    // if path.extension().unwrap().to_str().unwrap() == "zip" {
    //     unpack_zip(&file_path)?;
    // } else if path.extension().unwrap().to_str().unwrap() == "tar" {
    //     unpack_tar(&file_path)?;
    // }
    unpack_tar(path)?;
    Ok(())
}

async fn determine_file_path(url: &str, data: &[u8]) -> io::Result<PathBuf> {
    let file_name = url.split('/').last().unwrap_or("default.tar");
    let file_path = Path::new("./").join(file_name);
    let mut file = File::create(&file_path)?;
    file.write_all(data)?;
    Ok(file_path)
}

// fn unpack_zip(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
//     let file = File::open(file_path)?;
//     let mut archive = ZipArchive::new(file)?;

//     for i in 0..archive.len() {
//         let mut file = archive.by_index(i)?;
//         let outpath = file.sanitized_name();

//         if file.name().ends_with('/') {
//             std::fs::create_dir_all(&outpath)?;
//         } else {
//             if let Some(p) = outpath.parent() {
//                 if !p.exists() {
//                     std::fs::create_dir_all(&p)?;
//                 }
//             }
//             let mut outfile = File::create(&outpath)?;
//             std::io::copy(&mut file, &mut outfile)?;
//         }
//     }

//     println!("ZIP file extracted to {}", file_path.display());
//     Ok(())
// }

fn unpack_tar(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut archive: Archive<File> = Archive::new(file);

    let path = Path::new(file_path);
    let unpack_dir = path.parent().unwrap_or_else(|| Path::new("unpack_directory"));
    archive.unpack(unpack_dir)?;
    
    println!("TAR file extracted to {}", unpack_dir.display());
    Ok(())
}