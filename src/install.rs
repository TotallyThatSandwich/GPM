use crate::lib;

pub fn install(package: &String) {
    let mut packageVersion = "";
    let mut packageName = "";
    if package.contains("@V") {
        [packageName, packageVersion] = package.split("@V").collect::<Vec<&str>>().try_into().unwrap_or_default();
    } else {
        packageName = package;
        packageVersion = "latest";
    }

    println!("Package: `{}`, Version: `{}`", packageName, packageVersion);
}
