pub fn parse_package_input(input: &str) -> (String, String) {
    let package_version;
    let package_name;
    
    if input == "" {
        panic!("no input");
    }

    if input.contains("@V") {
        [package_name, package_version] = input
            .split("@V")
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap_or_default();
    } else {
        package_name = input;
        package_version = "latest";
    }

    if package_name == "" {
        panic!("no package_name");
    }
    if package_version == "" {
        panic!("no package_version");
    }

    (package_name.to_string(), package_version.to_string())
}
