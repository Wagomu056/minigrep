use minigrep::Config;

#[test]
fn load_from_config() {
    let args: Vec<String> = vec![
        "integration_test.rs".to_string(),
        "query".to_string(),
        "tests/test.txt".to_string()
    ];
    let config = Config::new(&args).unwrap();

    let result = minigrep::run(config).unwrap();
    assert_eq!(result, ());
}

