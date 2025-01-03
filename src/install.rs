use crate::lib;
use std::{
    fs::{self, ReadDir},
    path::{Iter, Path, PathBuf},
};

pub fn install(package: &str, file_path: Option<&Path>) {
    let (package_name, package_version) = lib::parse_package_input(package);

    let manifest_path;
    let clutter_path;
    let root_path;

    println!(
        "Package: `{}`, Version: `{}`",
        package_name, package_version
    );

    if let Some(file_path) = file_path {
        root_path = file_path;
    } else {
        root_path = Path::new(".");
    }

    let files = root_path
        .read_dir()
        .unwrap()
        .any(|file| file.unwrap().file_name() == "manifest.toml");
    if files {
        manifest_path = root_path.join("manifest.toml");
        clutter_path = root_path.join("clutter/");
    } else {
        fs::create_dir_all(root_path.join("clutter")).unwrap();
    }
}
