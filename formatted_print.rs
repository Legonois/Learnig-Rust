// rustc formatted_print.rs --out-dir bin

fn main() {
    println!("{} days", 31);

    println!("{0} {1}, {2}, {3}", "my", "name", "is", "Bond");

    println!("{key}: {value}",
             key="hello",
             value="world");

    let number: f64 = 3.141592;

    println!("pi is {}", number)
}