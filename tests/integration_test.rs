use py2rust::convert;

#[test]
fn test_simple_function() {
    let python_code = r#"
def add(a, b):
    return a + b
"#;

    let result = convert(python_code);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("fn add"));
    assert!(rust_code.contains("return"));
}

#[test]
fn test_variable_declaration() {
    let python_code = r#"
x = 5
"#;

    let result = convert(python_code);
    assert!(result.is_ok());
    let rust_code = result.unwrap();
    assert!(rust_code.contains("let x"));
}

#[test]
fn test_if_statement() {
    let python_code = r#"
if x > 5:
    y = 10
else:
    y = 20
"#;

    let result = convert(python_code);
    assert!(result.is_ok());
}
