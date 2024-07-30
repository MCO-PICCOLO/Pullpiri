/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */
mod decompress;
mod downloader;
mod file_handler;
pub mod parser;

pub async fn handle_package(
    package_name: &str,
) -> Result<parser::package::Package, Box<dyn std::error::Error>> {
    //url path
    let base_url = common::get_conf("DOC_REGISTRY");
    let full_url: String = format!("{}/packages/{}.tar", base_url, package_name);

    //save path
    let save_path: String = common::get_conf("YAML_STORAGE");
    let full_save_path = format!("{}/packages/{}.tar", save_path, package_name);

    //download, decompress
    println!("full url : {}", full_url);
    println!("full save path : {}", full_save_path);
    downloader::download(&full_url, &full_save_path).await?;
    decompress::decompress(&full_save_path).await?;

    //parsing
    let parsing_path = format!("{}/packages/{}", save_path, package_name);
    parser::package::package_parse(&parsing_path)
}

pub async fn handle_scenario(
    name: &str,
) -> Result<parser::scenario::Scenario, Box<dyn std::error::Error>> {
    let base_url = common::get_conf("DOC_REGISTRY");
    let full_url = format!("{}/scenarios/{}.yaml", base_url, name);

    let save_path: String = common::get_conf("YAML_STORAGE");
    let full_save_path = format!("{}/scenarios/{}.yaml", save_path, name);

    downloader::download(&full_url, &full_save_path).await?;
    parser::scenario::scenario_parse(&full_save_path)
}

#[cfg(test)]
mod tests {
    use crate::{handle_package, handle_scenario};

    #[tokio::test]
    async fn package_parse() {
        let result = handle_package("package-version1").await;
        println!("{:#?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn scenario_parse() {
        let result = handle_scenario("launch-scenario").await;
        println!("{:#?}", result);
        assert!(result.is_ok());
    }
}
