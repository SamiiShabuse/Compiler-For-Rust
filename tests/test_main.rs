use compiler_for_rust::validate_args;

#[test]
fn test_validate_args_no_args() {
    let args: Vec<String> = vec!["comp".to_string()];
    let result = validate_args(&args);
    assert!(result.is_err());
}

#[test]
fn test_validate_args_with_args() {
    let args: Vec<String> = vec!["comp".to_string(), "tokenize".to_string()];
    let result = validate_args(&args);
    assert!(result.is_ok());
}

