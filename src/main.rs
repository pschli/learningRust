// use std::cmp::Ordering;
// use std::io;

// use rand::Rng;

// fn main() {
//  ***** Chapter 2 *****
//
//  println!("Guess the number!");

// let secret_number = rand::thread_rng().gen_range(1..=100);

// loop {
//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line.");

//     if guess.trim() == "x" {
//         break;
//     }

//     let guess: u32 = match guess.trim().parse() {
//         Ok(num) => num,
//         Err(_) => continue,
//     };

//     println!("You guessed: {guess}");

//     match guess.cmp(&secret_number) {
//         Ordering::Less => println!("Too small!"),
//         Ordering::Equal => {
//             println!("You win!");
//             break;
//         }
//         Ordering::Greater => println!("Too big!"),
//     }
// }
//
// *********************************************************************
// ***** 3.1 *****
// let a: u8 = 250;
// let b: u8 = 10;

// let result = a.wrapping_add(b); // 250 + 10 = 4 (wraps around)
// println!("wrapping_add: {}", result);

// match a.checked_add(b) {
//     Some(v) => println!("checked_add: {}", v),
//     None => println!("checked_add overflowed"),
// }

// let (value, overflowed) = a.overflowing_add(b);
// println!(
//     "overflowing_add: value={}, overflowed={}",
//     value, overflowed
// );

// let result = a.saturating_add(b); // saturates at 255
// println!("saturating_add: {}", result);

// let small: u8 = 5;
// let result2 = small.saturating_sub(10); // saturates at 0
// println!("saturating_sub: {}", result2);
//
// *******************************************************************
// **** 3.2 ****

// let a = [1, 2, 3, 4, 5];

// println!("Please enter an array index.");

// let mut index = String::new();

// io::stdin()
//     .read_line(&mut index)
//     .expect("Failed to read line");

// let index: usize = index
//     .trim()
//     .parse()
//     .expect("Index entered was not a number");

// let element = a[index];

// println!("The value of the element at index {index} is: {element}");
// ***************************************************
// **** 4.2 ****
// The & is uses to create a reference, so ownership is not transferred. This is called borrowing.

fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // note the ampersands
    let s = format!("{} {}", m1, m2);
    println!("{}", s);
}

fn greet(g1: &String, g2: &String) {
    // note the ampersands
    println!("{} {}!", g1, g2);
}
// }
