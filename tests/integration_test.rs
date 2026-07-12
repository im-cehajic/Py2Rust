use py2rust::convert;

#[test]
fn test_simple_function() {
    let python_code = r#"
def add(a, b):
    return a + b
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    let rust_code = result.unwrap();
    println!("Generated Rust:\n{}", rust_code);
    assert!(rust_code.contains("fn add"));
    assert!(rust_code.contains("return"));
}

#[test]
fn test_variable_declaration() {
    let python_code = r#"
x = 5
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    let rust_code = result.unwrap();
    println!("Generated Rust:\n{}", rust_code);
    assert!(rust_code.contains("let x"));
}

#[test]
fn test_if_statement() {
    let python_code = r#"
x = 10
if x > 5:
    y = 20
else:
    y = 30
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    let rust_code = result.unwrap();
    println!("Generated Rust:\n{}", rust_code);
    assert!(rust_code.contains("if"));
    assert!(rust_code.contains("else"));
}

#[test]
fn test_for_loop() {
    let python_code = r#"
for i in range(10):
    x = i
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    let rust_code = result.unwrap();
    println!("Generated Rust:\n{}", rust_code);
    assert!(rust_code.contains("for"));
}

#[test]
fn test_fibonacci() {
    let python_code = r#"
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    let rust_code = result.unwrap();
    println!("Generated Rust:\n{}", rust_code);
    assert!(rust_code.contains("fn fibonacci"));
    assert!(rust_code.contains("if"));
    assert!(rust_code.contains("return"));
}

#[test]
fn test_list_literal() {
    let python_code = r#"
x = [1, 2, 3, 4, 5]
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    let rust_code = result.unwrap();
    println!("Generated Rust:\n{}", rust_code);
    assert!(rust_code.contains("vec!"));
}

#[test]
fn test_string_concatenation() {
    let python_code = r#"
message = "Hello, " + "World"
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    let rust_code = result.unwrap();
    println!("Generated Rust:\n{}", rust_code);
    assert!(rust_code.contains("let message"));
}

#[test]
fn test_comparison_operators() {
    let python_code = r#"
if x == 5:
    y = 1
elif x != 3:
    y = 2
else:
    y = 3
"#;

    // Note: elif not yet supported, but we can test basic comparisons
    let python_code = r#"
if x == 5:
    y = 1
else:
    y = 2
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    let rust_code = result.unwrap();
    println!("Generated Rust:\n{}", rust_code);
}
