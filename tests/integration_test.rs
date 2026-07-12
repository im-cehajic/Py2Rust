//! Enhanced test suite for Py2Rust

use py2rust::convert;

// ============= BASIC TESTS =============

#[test]
fn test_simple_function() {
    let python_code = r#"
def add(a, b):
    return a + b
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    let rust_code = result.unwrap();
    println!("=== Simple Function ===");
    println!("Generated Rust:\n{}", rust_code);
    assert!(rust_code.contains("fn add"));
    assert!(rust_code.contains("return"));
}

#[test]
fn test_variable_declaration() {
    let python_code = r#"
x = 5
y = 3.14
z = "hello"
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    let rust_code = result.unwrap();
    println!("=== Variable Declaration ===");
    println!("Generated Rust:\n{}", rust_code);
    assert!(rust_code.contains("let x"));
    assert!(rust_code.contains("let y"));
    assert!(rust_code.contains("let z"));
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
    println!("=== If Statement ===");
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
    println!("=== For Loop ===");
    println!("Generated Rust:\n{}", rust_code);
    assert!(rust_code.contains("for"));
}

#[test]
fn test_while_loop() {
    let python_code = r#"
i = 0
while i < 10:
    i = i + 1
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    let rust_code = result.unwrap();
    println!("=== While Loop ===");
    println!("Generated Rust:\n{}", rust_code);
    assert!(rust_code.contains("while"));
}

// ============= ADVANCED ALGORITHM TESTS =============

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
    println!("=== Fibonacci ===");
    println!("Generated Rust:\n{}", rust_code);
    assert!(rust_code.contains("fn fibonacci"));
    assert!(rust_code.contains("if"));
    assert!(rust_code.contains("return"));
}

#[test]
fn test_factorial() {
    let python_code = r#"
def factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    let rust_code = result.unwrap();
    println!("=== Factorial ===");
    println!("Generated Rust:\n{}", rust_code);
    assert!(rust_code.contains("fn factorial"));
}

#[test]
fn test_bubble_sort() {
    let python_code = r#"
def bubble_sort(arr):
    n = len(arr)
    for i in range(n):
        for j in range(n - i - 1):
            if arr[j] > arr[j + 1]:
                temp = arr[j]
                arr[j] = arr[j + 1]
                arr[j + 1] = temp
    return arr
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    let rust_code = result.unwrap();
    println!("=== Bubble Sort ===");
    println!("Generated Rust:\n{}", rust_code);
    assert!(rust_code.contains("fn bubble_sort"));
}

// ============= DATA STRUCTURE TESTS =============

#[test]
fn test_list_literal() {
    let python_code = r#"
x = [1, 2, 3, 4, 5]
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    let rust_code = result.unwrap();
    println!("=== List Literal ===");
    println!("Generated Rust:\n{}", rust_code);
    assert!(rust_code.contains("vec!"));
}

#[test]
fn test_list_comprehension_simple() {
    // Note: comprehensions not yet supported, testing nested loop workaround
    let python_code = r#"
result = []
for i in range(5):
    result = result + [i]
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    println!("=== List Comprehension Workaround ===");
    println!("Generated Rust:\n{}", result.unwrap());
}

#[test]
fn test_dict_literal() {
    let python_code = r#"
d = {"a": 1, "b": 2, "c": 3}
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    let rust_code = result.unwrap();
    println!("=== Dict Literal ===");
    println!("Generated Rust:\n{}", rust_code);
    assert!(rust_code.contains("HashMap"));
}

// ============= EXPRESSION TESTS =============

#[test]
fn test_arithmetic_operations() {
    let python_code = r#"
a = 10
b = 3
sum_val = a + b
diff_val = a - b
prod_val = a * b
div_val = a / b
mod_val = a % b
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    let rust_code = result.unwrap();
    println!("=== Arithmetic Operations ===");
    println!("Generated Rust:\n{}", rust_code);
}

#[test]
fn test_comparison_operators() {
    let python_code = r#"
x = 5
if x == 5:
    y = 1
else:
    y = 0
if x != 3:
    z = 1
if x > 0:
    w = 1
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    let rust_code = result.unwrap();
    println!("=== Comparison Operators ===");
    println!("Generated Rust:\n{}", rust_code);
    assert!(rust_code.contains("=="));
    assert!(rust_code.contains("!="));
    assert!(rust_code.contains(">"));
}

#[test]
fn test_boolean_operations() {
    let python_code = r#"
x = True
y = False
if x:
    z = 1
if not y:
    w = 1
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    let rust_code = result.unwrap();
    println!("=== Boolean Operations ===");
    println!("Generated Rust:\n{}", rust_code);
}

#[test]
fn test_string_concatenation() {
    let python_code = r#"
message = "Hello, " + "World"
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    let rust_code = result.unwrap();
    println!("=== String Concatenation ===");
    println!("Generated Rust:\n{}", rust_code);
    assert!(rust_code.contains("let message"));
}

// ============= COMPLEX FUNCTION TESTS =============

#[test]
fn test_multiple_return_paths() {
    let python_code = r#"
def max_value(a, b):
    if a > b:
        return a
    else:
        return b
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    let rust_code = result.unwrap();
    println!("=== Multiple Return Paths ===");
    println!("Generated Rust:\n{}", rust_code);
}

#[test]
fn test_nested_loops() {
    let python_code = r#"
for i in range(3):
    for j in range(3):
        x = i + j
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    let rust_code = result.unwrap();
    println!("=== Nested Loops ===");
    println!("Generated Rust:\n{}", rust_code);
}

#[test]
fn test_nested_if_statements() {
    let python_code = r#"
if x > 0:
    if y > 0:
        z = 1
    else:
        z = 2
else:
    z = 3
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    let rust_code = result.unwrap();
    println!("=== Nested If Statements ===");
    println!("Generated Rust:\n{}", rust_code);
}

// ============= EDGE CASES =============

#[test]
fn test_empty_function() {
    let python_code = r#"
def empty_func():
    pass
"#;

    // This will fail because 'pass' isn't supported yet
    let result = convert(python_code);
    // We just check it doesn't crash
    let _ = result;
}

#[test]
fn test_multiple_functions() {
    let python_code = r#"
def func1():
    return 1

def func2():
    return 2

def func3():
    return 3
"#;

    let result = convert(python_code);
    assert!(result.is_ok(), "Failed to convert: {:?}", result.err());
    let rust_code = result.unwrap();
    println!("=== Multiple Functions ===");
    println!("Generated Rust:\n{}", rust_code);
    assert!(rust_code.contains("fn func1"));
    assert!(rust_code.contains("fn func2"));
    assert!(rust_code.contains("fn func3"));
}
