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

// fn main() {
//     let mut v: Vec<String> = vec![String::from("Hello world")];
//     let mut s: String = v.remove(0);
//     s.push('!');
//     println!("{s}");
//     assert!(v.len() == 0);
// }

// ******************************************************************
// ***** Slices ****
// Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it is a non-owning pointer.

// fn main() {
//     fn first_word(s: &String) -> usize {
//         let bytes = s.as_bytes();

//         for (i, &item) in bytes.iter().enumerate() {
//             if item == b' ' {
//                 return i;
//             }
//         }
//         s.len()
//     }

//     let s = String::from("hello world");
//     let end1 = first_word(&s);

//     let hello: &str = &s[..end1]; // <- slice
//     let world: &str = &s[end1 + 1..s.len()]; // <- slice
//     let s2: &String = &s;

//     println!("{} {}", hello, world);
//     println!("{}", s2);
// }

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// fn main() {
//     let s = String::from("hello world");
//     let word = first_word(&s);
//     if s.len() > word.len() {
//         println!("the first word is: {}", word);
//     } else {
//         println!("There is no <space> in: {}", word);
//     }
// }
// slices also work with arrays
// *****************************************************************
// **** Structs ****

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// struct Color(i32, i32, i32); // <- Tuple Struct

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         // username: username,
//         // email: email,
//         email,    // <- init shorthand
//         username, // <- variable name matches key
//         sign_in_count: 1,
//     }
// }

// fn main() {
//     let mut user1 = build_user(
//         String::from("someone@example.com"),
//         String::from("someusername123"),
//     );

//     println!("{}", user1.email);
//     println!("{}", user1.username);
//     println!("{}", user1.active);
//     println!("{}", user1.sign_in_count);
//     let black = Color(0, 0, 0);
//     let Color(r, g, b) = &black; // <- destructuring the tuple struct
//     println!("{}, {}, {}", r, g, b);
//     let mail = &mut user1.email; // <- borrowing mutable reference
//     *mail = String::from("anotherEmail@mail.com"); // <- mutating
//     println!("{}", user1.email);
// }
// ********************************************
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: Rectangle) -> u32 {
//     dimensions.width * dimensions.height
// }
// *******************************************
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     dbg!(&rect1);
// }
//
// ***************************************************************
// ***** Methods *****
//
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     // <- method
//     fn area(&self) -> u32 { // <- self to reference the rectangle instance (borrowing, not taking ownership)
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area() // <- method all with instance.method()
//     );
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         (self.width > other.width && self.height > other.height)
//             || (self.width > other.height && self.height > other.width)
//     }

//     fn square(side: u32) -> Self {
//         // <- associated function
//         Self {
//             width: side,
//             height: side,
//         }
//     }
// }

// fn main() {
//     // let rect1 = Rectangle {
//     //     width: 30,
//     //     height: 50,
//     // };
//     // let rect2 = Rectangle {
//     //     width: 10,
//     //     height: 40,
//     // };
//     // let rect3 = Rectangle {
//     //     width: 60,
//     //     height: 45,
//     // };

//     // println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     // println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

//     let square1 = Rectangle::square(7);
//     println!("Square1 area: {} sqm", square1.area());
// }

// // The idiomatic way to define a constructor function is to make an associated function called new, but that is not enforced by the language.

// *****************************************************************
// ***** Methods and ownership *****
// #[derive(Copy, Clone)] // <- API authors must explicitly add #[derive(Copy)] to indicate that they expect their struct to always be Copy.
//     width: u32,
//     height: u32,
// }
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn set_width(&mut self, width: u32) {
//         self.width = width;
//     }

//     fn max(self, other: Self) -> Self {
//         let w = self.width.max(other.width);
//         let h = self.height.max(other.height);
//         Rectangle {
//             width: w,
//             height: h,
//         }
//     }
// }
// fn main() {
//     let mut rect = Rectangle {
//         width: 0,
//         height: 0,
//     };
//     println!("{}", rect.area());
//     rect.set_width(0);

//     let rect_ref = &rect;
//     // rect_ref.set_width(2);

//     let other_rect = Rectangle {
//         width: 1,
//         height: 1,
//     };
//     let max_rect = rect.max(other_rect);
//     println!("{}", rect.area());
// }
//
// ***********************************************************************************
// ***** Enums and Pattern Matching *****
//
