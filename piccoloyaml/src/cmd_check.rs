use std::env;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

fn remove_extension(name: &str) -> String {
    let stem = Path::new(name).file_stem().and_then(OsStr::to_str).unwrap();
    stem.to_string()
}

fn create_path_if_not_exists(path: &str) -> io::Result<()> {
    fs::create_dir_all(path)
}

fn convert_path_to_string(path: &str) -> String {
    let dir: &Path = Path::new(path);
    let dir_str: String = dir.to_str().unwrap_or_default().to_string();
    dir_str
}

fn create_kube_file(path: &str, file_name: &str) -> io::Result<File> {
    let dir_str = convert_path_to_string(path);
    let kube_name = format!("{}{}", dir_str, file_name);
    let kubefile: File = File::create(kube_name)?;
    Ok(kubefile)
}

fn copy_yaml_file(path: &str, file_name: &str) -> io::Result<()> {
    let src: std::path::PathBuf = env::current_dir()?.join(file_name);
    let dir_str: String = convert_path_to_string(path);
    let yaml_name: String = format!("{}{}", dir_str, file_name);
    fs::copy(&src, &yaml_name)?;
    Ok(())
}

fn write_content(path: &str, file: &mut File, file_name: &str) -> io::Result<()> {
    let dir_str = convert_path_to_string(path);
    let default_contents = "[Install]\nWantedBy=default.target\n\n[Unit]\nRequires=quadlet-demo-mysql.service\nAfter=quadlet-demo-mysql.service\n\n";
    let contents = format!("[Kube]\nYaml={}{}\n", dir_str, file_name);
    let combined = format!("{}{}", default_contents, contents);
    file.write_all(combined.as_bytes())?;
    Ok(())
}

pub fn command_check(input: &Vec<String>) {
    if input.len() >= 3 {
        if input[1] == "apply" {
            let origin_name = input[3].as_str();
            let path = "/etc/containers/systemd/";
            let name: String = remove_extension(origin_name);
            let file_name: String = format!("{}.kube", name);

            match create_path_if_not_exists(path) {
                Ok(_) => println!("Path created successfully."),
                Err(e) => println!("Failed to create path: {}", e),
            }

            let file: Result<File, io::Error> = create_kube_file(path, &file_name);
            match file {
                Ok(mut file) => {
                    println!("Kube File created successfully.");
                    match write_content(path, &mut file, origin_name) {
                        Ok(_) => println!("contents Write successfully."),
                        Err(e) => println!("contents Write Failed : {}", e),
                    }
                }
                Err(e) => println!("Kube Failed to create file: {}", e),
            }

            match copy_yaml_file(path, origin_name) {
                Ok(_) => println!("Yaml File copy successfully."),
                Err(e) => println!("Yaml Failed to copy : {}", e),
            }
        } else if input[1] == "delete" {
        } else {
        }
    }
}
