use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind, Result, Write};
use std::path::Path;

const SYSTEMD_FILE_PATH: &str = r#"/etc/containers/systemd/"#;
const CONTENTS_HEADER: &str = r#"[Unit]
Description=A kubernetes yaml based pingpong service
Before=local-fs.target

[Install]
# Start by default on boot
WantedBy=multi-user.target default.target

[Kube]
Yaml="#;

fn check_file_extension(name: &str) -> Result<()> {
    match Path::new(name).extension() {
        Some(ext) => {
            if "yaml" == ext {
                Ok(())
            } else {
                Err(Error::new(
                    ErrorKind::Other,
                    "Only 'yaml' file extension is allowed.",
                ))
            }
        }
        _ => Err(Error::new(ErrorKind::Other, "There is no file extension.")),
    }
}

fn create_dot_kube_file(name: &str) -> Result<()> {
    let kube_file_path = Path::new(name).with_extension("kube");
    let mut file = File::create(kube_file_path)?;
    file.write_all(format!("{}{}", CONTENTS_HEADER, name).as_bytes())?;
    Ok(())
}

/// input example  : ./my_pod.yaml
/// output example : /etc/containers/systemd/my_pod.yaml
///                  /etc/containers/systemd/my_pod.kube
pub fn process(cmd: &str, file_path: &str) -> Result<()> {
    if cmd == "apply" {
        let src_pathbuf = fs::canonicalize(file_path)?;
        let src = src_pathbuf.into_os_string().into_string().unwrap();
        let file_name = Path::new(&src).file_name().unwrap().to_str().unwrap();
        let dst = format!("{}{}", SYSTEMD_FILE_PATH, file_name);

        check_file_extension(file_name)?;
        fs::copy(&src, &dst)?;

        create_dot_kube_file(&dst)?;
        Ok(())
    } else if cmd == "delete" {
        Err(Error::new(
            ErrorKind::Other,
            "Currently 'delete' is not support",
        ))
    } else {
        Err(Error::new(
            ErrorKind::Other,
            format!("{} is not support", cmd),
        ))
    }
}
