pub fn parse_package_input(input: &String) -> (String, String){
    let mut packageVersion = "";
    let mut packageName = "";
    if input.contains("@V") {
        [packageName, packageVersion] = input.split("@V").collect::<Vec<&str>>().try_into().unwrap_or_default();
    } else {
        packageName = input;
        packageVersion = "latest";
    }
    return (packageName.to_string(), packageVersion.to_string())
}
