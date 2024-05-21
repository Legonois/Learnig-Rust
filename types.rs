

fn main() {
    // 1. Casting
    let decimal: f32 = 65.4321;

    let integer: u8 = decimal as u8;
    let chatacter: char = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, chatacter);

    let decimal: f32 = 300.3123;
    let integer: u8 = decimal as u8;

    // If the value exceeds the maximum bound, the result will be
    // the maximum bound value
    println!("Casting: {} -> {}", decimal, integer);

    // 2. Inference
    let elem = 5u8;

    let mut vec = Vec::new(); // a vector is a dynamic array
    let vec2: Vec<u8> = Vec::new(); // you can also specify the type

    // at this point the compiler doesn't know the type of vec
    vec.push(elem);

    println!("{:?}", vec);
    println!("{:?}", vec2);

    // 3. Aliasing
    type NanoSecond = u64;
    type Inch = u64;
    type U64 = u64;

    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    // They do NOT add any sort of extra type safety, they are just aliases
    println!("{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches);
}