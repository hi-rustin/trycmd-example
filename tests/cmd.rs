#[test]
fn test_cmd() {
    let t = trycmd::TestCases::new();
    let trycmd_example_binary = trycmd::cargo::cargo_bin("trycmd-example");
    t.register_bin("trycmd-example", &trycmd_example_binary);
    t.case("tests/cmd/*.toml");
}
