// Import statements
use std::convert::From;
use std::convert::Into;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// redundent because of the From trait
// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    // 1. From
    let num = Number::from(30i32);
    println!("My number is type {:?}", num);
    println!("My number is {:?}", num.value);

    // 2. Into
    let int = 5;
    let num: Number = int.into();
    println!("My number is type {:?}", num);
    println!("My number is {:?}", num.value);

    // "{:?}" means using the debug formatter

    // 3. TryFrom
    println!("{:?}", EvenNumber::try_from(8) == Ok(EvenNumber(8)));
    println!("{:?}", EvenNumber::try_from(5) == Err(()));

    // 4. TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    println!("{:?}", result);
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
    println!("{:?}", result);

    // 5. into a string
    let my_str = "hello";
    let my_string = String::from(my_str);
    println!("{:?}", my_string);

    // Custom formatter of a struct
    let circle = Circle { radius: 6 };
    println!("{}", circle);

    // 5. Parsing a string
    let parsed: u32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<u32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}

