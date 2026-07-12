//! Example: Data structure conversions

use py2rust::convert;

fn main() {
    let python_code = r#"
data = [1, 2, 3, 4, 5]
mappings = {"a": 1, "b": 2, "c": 3}

for item in data:
    x = item
"#;

    match convert(python_code) {
        Ok(rust_code) => {
            println!("Data Structures Example:");
            println!("=======================");
            println!("{}", rust_code);
        }
        Err(e) => eprintln!("Conversion failed: {}", e),
    }
}
