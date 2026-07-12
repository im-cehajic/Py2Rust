//! Example: Converting Python sorting algorithm to Rust

use py2rust::convert;

fn main() {
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

    match convert(python_code) {
        Ok(rust_code) => {
            println!("Python Code:");
            println!("=============");
            println!("{}", python_code);
            println!("\nConverted to Rust:");
            println!("==================");
            println!("{}", rust_code);
        }
        Err(e) => eprintln!("Conversion failed: {}", e),
    }
}
