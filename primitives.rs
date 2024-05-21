// Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
   // Signed integers can store positive and negative values.

// Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
   // Unsigned integers can store only positive values.

// Floating point: f32, f64
// char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
// bool either true or false
// The unit type (), whose only possible value is an empty tuple: ()

fn print_type_of<T>(_: &T) { // https://stackoverflow.com/questions/21747136/how-do-i-print-in-rust-the-type-of-a-variable
    println!("{}", std::any::type_name::<T>())
}

fn main() {

    // let is immutable by default
    // to make it mutable, use let mut explicitly

    let logical: bool = true;
    print_type_of(&logical); // & means reference (pointer)]
    
    let a_float: f64 = 1.0;  // Regular annotation
    print_type_of(&a_float);
    let an_integer   = 5i32; // Suffix annotation
    print_type_of(&an_integer);

    // Infered types
    let default_float   = 3.0;
    print_type_of(&default_float);
    let default_integer = 7;
    print_type_of(&default_integer);
    // default_integer = 8; // error: cannot assign twice to immutable variable

    let mut mutable = 12; // Mutable `i32`
    print_type_of(&mutable);
    println!("mutable: {}", mutable);

    mutable = 21; // OK
    println!("mutable: {}", mutable);

    // print the type of the variable
    println!("logical: {:?}", logical);


    // Tuple
    let big_tuple: (i32, f64, u8) = (500, 6.4, 1);
    print_type_of(&big_tuple);

    let (x, y, z) = big_tuple;
    println!("x: {}, y: {}, z: {}", x, y, z);
    println!("x: {}, y: {}, z: {}", big_tuple.0, big_tuple.1, big_tuple.2); // Access valuse by using .index

    // Arrays!
           /* [type; size] */
    let array: [i32; 5]  = [1, 2, 3, 4, 5];
    println!("array: {:?}", array);

    println!("second element: {}", array[1]);

    // Array of 5 elements, all elements are 3
    let array5 = [5; 5];
    println!("array5: {:?}", array5);

    // for (i = 0; i < array5.len(); i++) {
    //     println!("array5[{}]: {}", i, array5[i]);
    // }

    for i in 0..array.len() {
        println!("array[{}]: {}", i, array[i]);
    }
}