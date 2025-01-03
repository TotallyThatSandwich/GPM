use gpm::parse_package_input;

#[test]
fn test_parse_package_input_1() {
    // Test with a package name and version
    let (name, version) = parse_package_input("my-package@V1.2.3");
    assert_eq!(name, "my-package");
    assert_eq!(version, "1.2.3");
} 

#[test]
fn test_parse_package_input_2() {
    // Test with only a package name
    let (name, version) = parse_package_input("my-package");
    assert_eq!(name, "my-package");
    assert_eq!(version, "latest");
}

#[test]
#[should_panic]
fn test_parse_package_input_3() {
    // Test with an invalid input
    let (name, version) = parse_package_input("@V1.2.3");
}
