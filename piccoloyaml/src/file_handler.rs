use std::ffi::OsStr;
use std::fs;
use std::io::{Error, ErrorKind, Result, Write};
use std::path::Path;

macro_rules! Err {
    ($x:expr) => {
        Err(Error::new(ErrorKind::Other, $x))
    };
}

const SYSTEMD_FILE_PATH: &str = r#"/etc/containers/systemd/"#;
const CONTENTS_HEADER: &str = r#"[Unit]
Description=A kubernetes yaml based pingpong service
Before=local-fs.target

[Install]
# Start by default on boot
WantedBy=multi-user.target default.target

[Kube]
Yaml="#;

fn get_absolute_file_path(name: &str) -> Result<String> {
    let file = Path::new(name);
    if !file.is_file() {
        return Err!(format!("Not found or invalid path - {}", name));
    }

    if matches!(file.extension(), Some(x) if x == OsStr::new("yaml")) {
        let absolute_path = fs::canonicalize(file)?;
        Ok(absolute_path.into_os_string().into_string().unwrap())
    } else {
        Err!("Unsupported file extension")
    }
}

fn create_dst_file(src_yaml_path: &str, dst_yaml_path: &str) -> Result<()> {
    let mut file = fs::File::create(Path::new(dst_yaml_path).with_extension("kube"))?;

    file.write_all(format!("{}{}", CONTENTS_HEADER, dst_yaml_path).as_bytes())?;
    fs::copy(src_yaml_path, dst_yaml_path)?;
    Ok(())
}

fn delete_dst_files(dst_yaml_path: &str) -> Result<()> {
    fs::remove_file(Path::new(dst_yaml_path))?;
    fs::remove_file(Path::new(dst_yaml_path).with_extension("kube"))?;
    Ok(())
}

/// input example  : ./my_pod.yaml
/// output example : /etc/containers/systemd/my_pod.yaml
///                  /etc/containers/systemd/my_pod.kube
pub fn handle(cmd: &str, src_file_path: &str) -> Result<()> {
    let src = get_absolute_file_path(src_file_path)?;
    let file_name = Path::new(&src).file_name().unwrap().to_str().unwrap();
    let dst = format!("{}{}", SYSTEMD_FILE_PATH, file_name);

    if cmd == "apply" {
        create_dst_file(&src, &dst)?;
    } else if cmd == "delete" {
        delete_dst_files(&dst)?;
    } else {
        return Err!(format!("{} is not support", cmd));
    }
    Ok(())
}
