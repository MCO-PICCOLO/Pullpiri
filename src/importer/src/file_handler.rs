/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
pub fn get_absolute_file_path(path: &str) -> Result<PathBuf, Box<dyn Error>> {
    let file = Path::new(path);
    if !file.is_file() {
        return Err(format!("Not found or invalid path - {path}").into());
    }

    if matches!(file.extension(), Some(x) if x == OsStr::new("yaml")) {
        Ok(fs::canonicalize(file)?)
    } else {
        Err("Unsupported file extension".into())
    }
}

fn make_kube_file(dir: &str, name: &str) -> Result<(), Box<dyn Error>> {
    let kube_file_path = format!("{}/{}.kube", dir, name);
    let yaml_file_path = format!("{}/{}.yaml", dir, name);
    let mut kube_file = fs::File::create(kube_file_path)?;

    let kube_contents = format!(
        r#"[Unit]
Description=A kubernetes yaml based {} service
Before=local-fs.target

[Install]
# Start by default on boot
WantedBy=multi-user.target default.target

[Kube]
Yaml={}
"#,
        name, yaml_file_path
    );
    kube_file.write_all(kube_contents.as_bytes())?;

    Ok(())
}

fn make_yaml_file(dir: &str, name: &str, model: &str) -> Result<(), Box<dyn Error>> {
    let yaml_file_path = format!("{}/{}.yaml", dir, name);
    let mut yaml_file = fs::File::create(yaml_file_path)?;

    yaml_file.write_all(model.as_bytes())?;

    Ok(())
}

pub fn perform(
    model_name: &str,
    parsed_model_str: &str,
    package_name: &str,
) -> Result<(), Box<dyn Error>> {
    let directory = format!(
        "{}/packages/{}/{}",
        common::get_conf("YAML_STORAGE"),
        package_name,
        model_name
    );
    fs::create_dir_all(&directory)?;

    make_kube_file(&directory, model_name)?;
    make_yaml_file(&directory, model_name, parsed_model_str)?;

    Ok(())
}
