// Integration test for complete conversion pipeline

use py2rust::convert;

#[test]
fn test_complete_pipeline_simple() {
    let python_code = r#"
def greet(name):
    return "Hello, " + name

result = greet("World")
"#;

    let rust_result = convert(python_code);
    assert!(rust_result.is_ok());
    let rust_code = rust_result.unwrap();
    
    println!("Python Input:");
    println!("{}", python_code);
    println!("\nRust Output:");
    println!("{}", rust_code);
    
    assert!(rust_code.contains("fn greet"));
    assert!(rust_code.contains("fn main"));
}

#[test]
fn test_complete_pipeline_complex() {
    let python_code = r#"
def is_prime(n):
    if n < 2:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False
    for i in range(3, n):
        if n % i == 0:
            return False
    return True

primes = []
for num in range(10):
    if is_prime(num):
        primes = primes + [num]
"#;

    let rust_result = convert(python_code);
    assert!(rust_result.is_ok());
    let rust_code = rust_result.unwrap();
    
    println!("Complex Pipeline Test:");
    println!("Rust Output:\n{}", rust_code);
    
    assert!(rust_code.contains("fn is_prime"));
    assert!(rust_code.contains("fn main"));
    assert!(rust_code.contains("for"));
    assert!(rust_code.contains("if"));
}
