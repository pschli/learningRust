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
// }
// ***************************************************
// **** 4.2 ****
// The & is uses to create a reference, so ownership is not transferred. This is called borrowing.

// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("world");
//     greet(&m1, &m2); // note the ampersands
//     let s = format!("{} {}", m1, m2);
//     println!("{}", s);
// }

// fn greet(g1: &String, g2: &String) {
//     // note the ampersands
//     println!("{} {}!", g1, g2);
// }
// ***********************
// **** Dereferencing ****
//

// fn main() {
//     let mut x: Box<i32> = Box::new(1);
//     let a: i32 = *x; // *x reads the heap value, so a = 1
//     *x += 1; // *x on the left-side modifies the heap value,
//     //     so x points to the value 2

//     let r1: &Box<i32> = &x; // r1 points to x on the stack
//     let b: i32 = **r1; // two dereferences get us to the heap value

//     let r2: &i32 = &*x; // r2 points to the heap value directly
//     let c: i32 = *r2; // so only one dereference is needed to read it

//     let x: Box<i32> = Box::new(-1);
//     let x_abs1 = i32::abs(*x); // explicit dereference
//     let x_abs2 = x.abs(); // implicit dereference
//     assert_eq!(x_abs1, x_abs2);

//     let r: &Box<i32> = &x;
//     let r_abs1 = i32::abs(**r); // explicit dereference (twice)
//     let r_abs2 = r.abs(); // implicit dereference (twice)
//     assert_eq!(r_abs1, r_abs2);

//     let s = String::from("Hello");
//     let s_len1 = str::len(&s); // explicit reference
//     let s_len2 = s.len(); // implicit reference
//     assert_eq!(s_len1, s_len2);
// }

// ***********************
// **** Aliasing ****
// Aliasing is accessing the same data through different variables.
// Does not compile:
//

// let mut v: Vec<i32> = vec![1, 2, 3];
// let num: &i32 = &v[2];
// v.push(4); //
// println!("Third element is {}", *num);

// Pointer Safety Principle: data should never be aliased and mutated at the same time.
// Data can be aliased. Data can be mutated. But data cannot be both aliased and mutated.

// **** Fixing ownership ****
//
// fn return_a_string() -> &String {
//     let s = String::from("Hello world");
//     &s  <- This doesn't work, because s doesn't live long enough
// }
// //
// fn return_a_string() -> String {
//     let s = String::from("Hello world");
//     s // This does work because it transfers ownership
// }

// fn return_a_string() -> &'static str {
//     // The 'static string literal lives forever
//     "Hello world"
// }

// use std::rc::Rc; // <- Garbage collection at runtime
// fn return_a_string() -> Rc<String> {
//     let s = Rc::new(String::from("Hello world"));
//     Rc::clone(&s)
// }

// fn return_a_string(output: &mut String) {
//     // <- making the caller responsible for allocation
//     output.replace_range(.., "Hello world");
// }

fn main() {
    let mut v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v.remove(0);
    s.push('!');
    println!("{s}");
    assert!(v.len() == 0);
}
