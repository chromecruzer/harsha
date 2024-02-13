#[warn(dead_code)]

use std::string::String;
//use std::{ops::Index, vec::Vec};


fn main() {

    
    // oops and collection tricks
    println!("Hello, world!");

    // while loop

    let arr_2: [u32; 9] = [1, 1, 1, 1, 1, 1, 1, 1, 1];
    let mut loop_idx: usize = 0;
    while loop_idx < arr_2.len() {
        println!("arr {}", arr_2[loop_idx]);
        loop_idx += 1
    }

    // forloop

    let arr_1: [u32; 3] = [1, 2, 3];
    for val in arr_1.iter() {
        println!("for_arr{}", val);
    }

    // tuples in rust

    let my_tuple: (u8, String, f64) = (56, "maari".to_string(), 5000000_0.00);
    let formated = format!("{:?}", my_tuple);
    println!("{}", formated);

    // String push

    let mut stg = String::new();
    stg.push('A');
    stg.push_str("Puthu doli");
    for word in stg.split_whitespace() {
        println!("{word}");

        let stg2 = stg.replace("A", "Hari ");
        println!("{stg2}")
    }

    // String of Random different characters
    // also Vec! vectors similar to js array

    let s3 = String::from(" n ddsaf ff as d fd af ");
    let mut v1: Vec<char> = s3.chars().collect();
    v1.sort();
    v1.dedup();
    for val in v1 {
        println!("{}", val as i8); // just an example
    }

    // Using String
    let my_string = String::from("Hello, Rust!");
    let bytes_from_string: &[u8] = my_string.as_bytes();   // as_bytes() in rust works only for str and string which utf-8
    println!("Bytes from String: {:?}", bytes_from_string);

    // Using &str
    let my_str: &str = "Hello, Rust!";
    let bytes_from_str: &[u8] = my_str.as_bytes();
    println!("Bytes from &str: {:?}", bytes_from_str);


    // clear()
    let mut numbers = vec![1, 2, 3, 4, 5];

    println!("Original Vec: {:?}", numbers);

    // Clear the Vec
    numbers.clear();

    println!("Vec after clear(): {:?}", numbers);


    // castings 

    let int_8:u8  = 5;
    let int1_8:u8  = 10;
    let int_all: u128 = (int_8 as u128) + (int1_8 as u128);
    println!("{}",int_all);


    // enums .. struct .. impl

    #[allow(dead_code)]

    enum Days {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,

    }

    impl Days  {
        fn the_weekend(&self)-> bool {
            match self{
                Days::Saturday | Days::Sunday => true,
                _ => false
            }
        }
    }

    let today:Days = Days::Monday;
    match today{
        Days::Monday => println!("Every fuking individual hate that !!"),
        _ => println!("no data passed"),
    };

    println!("the day of the week {}", today.the_weekend());



    // vectors are like arrays they can grow if muttable and it save values of same type only 
    
    let _val:Vec<i32> = Vec::new();
    let  mut val2 = vec![1,2,3,4,5];
    val2.push(90);
    println!("the value of 2nd index is {}", val2[2]);  // val2.index(2)  // also applicable 
    let _third:&i32 = &val2[1];
    match val2.get(1)
    {
       Some(third) => println!("the value is {}", third),
         None => println!("nothing"),
    }

    for i in &mut val2 {
        *i += 2
    };

    for i in &val2{
        println!("val is {}", i)
    };

    println!("pop: {:?}", val2.pop());
    // LEN()
    println!("length: {}", val2.len())






    
} 


// use std::fmt::Debug;

// // Define a custom error type for the result
// #[derive(Debug)]
// enum ArithmeticError {
//     DivisionByZero,
//     UnknownOperator(char),
// }

// // Result type for the arithmetic operation
// type ArithmeticResult<T> = Result<T, ArithmeticError>;

// // Function to perform arithmetic operations
// fn perform_arithmetic_operation(operand1: f64, operator: char, operand2: f64) -> ArithmeticResult<f64> {
//     match operator {
//         '+' => Ok(operand1 + operand2),
//         '-' => Ok(operand1 - operand2),
//         '*' => Ok(operand1 * operand2),
//         '/' => {
//             if operand2 != 0.0 {
//                 Ok(operand1 / operand2)
//             } else {
//                 Err(ArithmeticError::DivisionByZero)
//             }
//         }
//         _ => Err(ArithmeticError::UnknownOperator(operator)),
//     }
// }

// fn main() {
//     // Example usage
//     match perform_arithmetic_operation(5.0, '+', 3.0) {
//         Ok(result) => println!("Result: {}", result),
//         Err(err) => eprintln!("Error: {:?}", err),
//     }

//     match perform_arithmetic_operation(8.0, '/', 0.0) {
//         Ok(result) => println!("Result: {}", result),
//         Err(err) => eprintln!("Error: {:?}", err),
//     }

//     match perform_arithmetic_operation(4.0, '%', 2.0) {
//         Ok(result) => println!("Result: {}", result),
//         Err(err) => eprintln!("Error: {:?}", err),
//     }
// }






// fn main() {
//     // 1. Create a vector with initial values
//     let original_vec = vec![1, 2, 3];

//     // 2. Take a mutable reference to the original vector
//     let mut reference_to_vec = &original_vec;

//     // 3. Modify the original vector through the mutable reference
//     for i in reference_to_vec.iter_mut() {
//         *i += 10; // Increment each element by 10
//     }

//     // 4. Shadow the original vector with a new vector
//     let original_vec = vec![4, 5, 6];

//     // 5. Convert the reference to an owned vector using `to_owned()`
//     let cloned_vec = reference_to_vec.to_owned();

//     // 6. Print the shadowed original vector
//     println!("Original vector (shadowed): {:?}", original_vec);

//     // 7. Print the cloned vector
//     println!("Cloned vector: {:?}", cloned_vec);
// }
 