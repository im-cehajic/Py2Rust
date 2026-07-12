//! Example: Mathematical functions

use py2rust::convert;

fn main() {
    let python_code = r#"
def power(base, exp):
    result = 1
    for i in range(exp):
        result = result * base
    return result

def is_even(n):
    if n % 2 == 0:
        return True
    else:
        return False

def absolute_value(n):
    if n < 0:
        return n * -1
    else:
        return n
"#;

    match convert(python_code) {
        Ok(rust_code) => {
            println!("Mathematical Functions Example:");
            println!("===============================");
            println!("{}", rust_code);
        }
        Err(e) => eprintln!("Conversion failed: {}", e),
    }
}
