use crate::lib;

pub fn install(package: &String) {
    let (packageName, packageVersion) = lib::parse_package_input(package);

    println!("Package: `{}`, Version: `{}`", packageName, packageVersion);
}
