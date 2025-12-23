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
// fn main() {
// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// let home = IpAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// };

// let loopback = IpAddr {
//     kind: IpAddrKind::V6,
//     address: String::from("::1"),
// };
// }
// ***** Data inside the enum *****
// fn main() {
//     enum IpAddr {
//         V4(String),
//         V6(String),
//     }

//     let home = IpAddr::V4(String::from("127.0.0.1"));

//     let loopback = IpAddr::V6(String::from("::1"));
// }
// you can put any kind of data inside an enum variant: strings, numeric types, or structs, for example.
// You can even include another enum!
// enums can implement methods
// ***** Option<T> *****
//
//
// fn main() {
//     let some_number = Some(5);
//     let some_char = Some('e');

//     let absent_number: Option<i32> = None;
// }

// The type of some_number is Option<i32>. The type of some_char is Option<char>, which is a different type.
// Rust can infer these types because we’ve specified a value inside the Some variant.
// For absent_number, Rust requires us to annotate the overall Option type:
// the compiler can’t infer the type that the corresponding Some variant will hold by looking only at a None value.
// Here, we tell Rust that we mean for absent_number to be of type Option<i32>.
// *********************************************************************************
// ***** The match Control Flow Construct *****

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState), // <- binding
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {state:?}!"); // <- bindung to a value
//             25
//         }
//     }
// }

// fn main() {
//     value_in_cents(Coin::Quarter(UsState::Alaska));
// }

// ***** matching with Option<T> *****

// fn main() {
//     fn plus_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             // <- the arms’ patterns must cover all possibilities. Matches in Rust are exhaustive
//             None => None,
//             Some(i) => Some(i + 1),
//         }
//     }

//     let five = Some(5);
//     let six = plus_one(five); // <- adds one
//     let none = plus_one(None); // <- returns None

//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         //    other => move_player(other), // this does bind all other cases
//         //    _ => reroll(), // _ is a special pattern that matches any value and does not bind to that value.
//         _ => (), // do nothing
//     }

//     fn add_fancy_hat() {}
//     fn remove_fancy_hat() {}
//     fn reroll() {}
//     fn move_player(num_spaces: u8) {}
// }
// ***********************************************************************
// ***** Concise Control Flow with if let and let else *********************
//
// fn main() {
//     let config_max = Some(3u8);
//     if let Some(max) = config_max {
//         // this is the same as matching with a non binding default case
//         println!("The maximum is configured to be {max}");
//     }
// }
// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn main() {
//     let coin = Coin::Penny;
//     let mut count = 0;
//     if let Coin::Quarter(state) = coin {
//         // <- see above, same functionality
//         println!("State quarter from {state:?}!");
//     } else {
//         count += 1;
//     }
// }

// #[derive(Debug)] // so we can inspect the state in a minute
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }

// impl UsState {
//     fn existed_in(&self, year: u16) -> bool {
//         match self {
//             UsState::Alabama => year >= 1819,
//             UsState::Alaska => year >= 1959,
//             // -- snip --
//         }
//     }
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn describe_state_quarter(coin: Coin) -> Option<String> {
//     let state = if let Coin::Quarter(state) = coin {
//         // <- produces a value
//         state
//     } else {
//         return None;
//     };

//     if state.existed_in(1900) {
//         // <- uses the value (state) conditionally => no nesting
//         Some(format!("{state:?} is pretty old, for America!"))
//     } else {
//         Some(format!("{state:?} is relatively new."))
//     }
// }

// fn describe_state_quarter(coin: Coin) -> Option<String> {
//     let Coin::Quarter(state) = coin else { // let.. else binds the pattern from the outer scope if it matches
//         return None;
//     };

//     if state.existed_in(1900) { // <- we can use state here
//         Some(format!("{state:?} is pretty old, for America!"))
//     } else {
//         Some(format!("{state:?} is relatively new."))
//     }
// }

// fn main() {
//     if let Some(desc) = describe_state_quarter(Coin::Quarter(UsState::Alaska)) {
//         println!("{desc}");
//     }
// }
//
// Library crates don’t have a main function, and they don’t compile to an executable.
// Instead, they define functionality intended to be shared with multiple projects.
// For example, the rand crate we used in Chapter 2 provides functionality that generates random numbers.
// Most of the time when Rustaceans say “crate,” they mean library crate,
// and they use “crate” interchangeably with the general programming concept of a “library.”
//
// A package can contain as many binary crates as you like, but at most only one library crate.
// A package must contain at least one crate, whether that’s a library or binary crate.
//
// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }
// Modules let us organize code within a crate for readability and easy reuse.
// Modules also allow us to control the privacy of items because code within a module is private by default.
// Private items are internal implementation details not available for outside use.
// We can choose to make modules and the items within them public, which exposes them to allow external code to use and depend on them.
// In Rust, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default.
// If you want to make an item like a function or struct private, you put it in a module.
//
// mod front_of_house {
//     pub mod hosting {
//         // <- needs pub to access
//         pub fn add_to_waitlist() {} // <- needs pub to access
//     }
// }

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }
//
// We can also use pub to designate structs and enums as public, but there are a few extra details
// to the usage of pub with structs and enums. If we use pub before a struct definition, we make the struct public,
// but the struct’s fields will still be private. We can make each field public or not on a case-by-case basis.
//
// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     // Order a breakfast in the summer with Rye toast.
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     // Change our mind about what bread we'd like.
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);

//     // The next line won't compile if we uncomment it; we're not allowed
//     // to see or modify the seasonal fruit that comes with the meal.
//     // meal.seasonal_fruit = String::from("blueberries"); // <- doesn't work, because seasonal_fruit is private
// }

// mod back_of_house {
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }

// pub fn eat_at_restaurant() {
//     let order1 = back_of_house::Appetizer::Soup; // <- no problem here, all variants of an enum are public if the enum is.
//     let order2 = back_of_house::Appetizer::Salad;
// }
//
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting;
// // use crate::front_of_house::hosting::add_to_waitlist;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist(); // <- this is idiomatic, because the origin of the function is clear.
//     //    add_to_waitlist();
// }

// // On the other hand, when bringing in structs, enums, and other items with use, it’s idiomatic to specify the full path.
// // The exception to this idiom is if we’re bringing two items with the same name into scope with use statements,
// // because Rust doesn’t allow that.
// //
// use std::collections::HashMap;

// fn main() {
//     let mut map = HashMap::new();
//     map.insert(1, 2);
// }

// use std::fmt::Result;
// use std::io::Result as IoResult; // <- solves the naming conflict

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub use crate::front_of_house::hosting; // <- bringing into scope

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }

// Before this change, external code would have to call the add_to_waitlist function by using the
// path restaurant::front_of_house::hosting::add_to_waitlist(), which also would have required the front_of_house module
// to be marked as pub. Now that this pub use has re-exported the hosting module from the root module,
// external code can use the path restaurant::hosting::add_to_waitlist() instead.
//
// Note that the standard std library is also a crate that’s external to our package.
// Because the standard library is shipped with the Rust language, we don’t need to change Cargo.toml to include std.
// But we do need to refer to it with use to bring items from there into our package’s scope.
// For example, with HashMap we would use this line:

// use std::collections::HashMap;

// // If we’re using multiple items defined in the same crate or same module, we can use nested paths to bring them into scope in one line.

// use std::io::{self, Write};
// use std::{cmp::Ordering, io}; // <- self refers to std::io - This line brings std::io and std::io::Write into scope.

// // If we want to bring all public items defined in a path into scope, we can specify that path followed by the * glob operator:

// use std::collections::*;
// ***********************************************************************************************
// ***** Vectors *****
//

// fn main() {
//     let mut v = Vec::new();

//     v.push(5);
//     v.push(6);
//     v.push(7);
//     v.push(8);

//     let third: &i32 = &v[2];
//     println!("The third element is {third}");

//     let third: Option<&i32> = v.get(2); // <- this is safe agains out of bounds
//     match third {
//         Some(third) => println!("The third element is {third}"),
//         None => println!("There is no third element."),
//     }

//     for i in &v {
//         // <- iterating
//         println!("{i}");
//     }

//     let mut v2 = vec![100, 32, 57];
//     for i in &mut v2 {
//         *i += 50;
//         println!("{i}");
//     }
// }
//
// ******** iterators ******
// fn main() {
//     use std::slice::Iter;
//     let mut v: Vec<i32> = vec![1, 2];
//     let mut iter: Iter<'_, i32> = v.iter();
//     let n1: &i32 = iter.next().unwrap();
//     let n2: &i32 = iter.next().unwrap();
//     let end: Option<&i32> = iter.next();
// }

// Observe that the iterator iter is a pointer that moves through each element of the vector.
// The next method advances the iterator and returns an optional reference to the previous element,
// either Some (which we unwrap) or None at the end of the vector. Also: to use iterators safely,
// Rust does not allow you to add or remove elements from the vector during iteration.
// One way to iterate over a vector without using a pointer is with a range:

// let mut v: Vec<i32>        = vec![1, 2];
// let mut iter: Range<usize> = 0 .. v.len();
// let i1: usize              = iter.next().unwrap();
// let n1: &i32               = &v[i1];

// ***** enums in vectors *****
//
// fn main() {
//     enum SpreadsheetCell {
//         Int(i32),
//         Float(f64),
//         Text(String),
//     }

//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Text(String::from("blue")),
//         SpreadsheetCell::Float(10.12),
//     ];
// }
// ********************************************************************************
// ***** Strings *******************************
//
// We discuss strings in the context of collections because strings are implemented as a collection of bytes.
// The String type, which is provided by Rust’s standard library rather than coded into the core language,
// is a growable, mutable, owned, UTF-8 encoded string type. When Rustaceans refer to “strings” in Rust,
// they might be referring to either the String or the string slice &str types, not just one of those types.
//
// fn main() {
//     let mut s = String::new();

//     let data = "initial contents";

//     let s = data.to_string();

//     // The method also works on a literal directly:
//     let s = "initial contents".to_string();

//     let s = String::from("initial contents");

//     let mut s = String::from("foo");
//     s.push_str("bar"); // <- The push_str method takes a string slice because we don’t necessarily want to take ownership of the parameter.

//     let s1 = String::from("Hello, ");
//     let s2 = String::from("world!");
//     let s3 = s1 + &s2; // <- note s1 has been moved here and can no longer be used

//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");

//     let s = format!("{s1}-{s2}-{s3}");

//     println!("{s1}");
//     println!("{s}");
//     // let h = s1[0]; // <- Rust strings don’t support indexing, because their length is in bytes
//     //                      and UTF-8 characters can take more than one byte per letter
//     match s1.chars().nth(0) {
//         Some(c) => println!("{c}"),
//         None => {}
//     };

//     for c in "Зд".chars() {
//         println!("{c}");
//     }

//     for b in "Зд".bytes() {
//         println!("{b}");
//     }
// }
//
// *********************************************************************************************************
// ***** Hashmaps *****
//
// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);

//     let team_name = String::from("Blue");
//     let score = scores.get(&team_name).copied().unwrap_or(0);

//     println!("Blue: {score}");

//     scores.insert(String::from("Blue"), 25); // <- overwriting

//     scores.entry(String::from("Yellow")).or_insert(60); // <- not updating, if key exists
//     scores.entry(String::from("Red")).or_insert(50); // <- new entry, if key doesn't exist

//     for (k, v) in scores {
//         println!("{k}: {v}")
//     }

//     let field_name = String::from("Favorite color");
//     let field_value = String::from("Blue");

//     let mut map = HashMap::new();
//     map.insert(field_name, field_value);
//     // println!("{field_name}"); // <- moved by insert()

//     let text = "hello world wonderful world";

//     let mut map1 = HashMap::new();

//     for word in text.split_whitespace() {
//         let count = map1.entry(word).or_insert(0);
//         *count += 1;
//     }

//     println!("{map1:?}");
// }
// *****************************************************************************
// ***** Errors *****
//

// fn main() {
//     panic!("crash and burn");
// }

// fn main() {
//     let v = vec![1, 2, 3];

//     v[99]; // panic
// }

// ***** Recoverable Errors with Result *****

// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let greeting_file_result = File::open("hello.txt");

//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {e:?}"),
//             },
//             _ => {
//                 panic!("Problem opening the file: {error:?}");
//             }
//         },
//     };
// }
//
//  ***** unwrap & expect

// use std::fs::File;

// fn main() {
//     let greeting_file = File::open("hello.txt").unwrap(); // <- panic! on error
// }

// use std::fs::File;

// fn main() {
//     let greeting_file =
//         File::open("hello.txt").expect("hello.txt should be included in this project"); // <- panic on error with overwritten message
// }

// #![allow(unused)]
// fn main() {
//     use std::fs::File;
//     use std::io::{self, Read};

//     fn read_username_from_file() -> Result<String, io::Error> {
//         // function returns Result<T, E>
//         let username_file_result = File::open("hello.txt");

//         let mut username_file = match username_file_result {
//             Ok(file) => file,
//             Err(e) => return Err(e), // <- returns an Error
//         };

//         let mut username = String::new();

//         match username_file.read_to_string(&mut username) {
//             Ok(_) => Ok(username), // <- on success returns a string
//             Err(e) => Err(e),      // <- returns an error
//         }
//     }
// }
// It’s up to the calling code to decide what to do with those values.
// If the calling code gets an Err value, it could call panic! and crash the program, use a default username,
// or look up the username from somewhere other than a file, for example.

// #![allow(unused)]
// fn main() {
//     use std::fs::File;
//     use std::io::{self, Read};

//     fn read_username_from_file() -> Result<String, io::Error> {
//         let mut username_file = File::open("hello.txt")?; // <- The ? placed after a Result value is defined to work in almost the same way
//         //    as the match expressions we defined to handle the Result values in the example above.
//         let mut username = String::new();
//         username_file.read_to_string(&mut username)?;
//         Ok(username) // <- If the value of the Result is an Ok, the value inside the Ok will get returned from this expression,
// and the program will continue. If the value is an Err, the Err will be returned from the whole function
// as if we had used the return keyword so the error value gets propagated to the calling code.
//
// There is a difference between what the match expression does and what the ? operator does:
// error values that have the ? operator called on them go through the from function, defined in the From trait
// in the standard library, which is used to convert values from one type into another. When the ? operator calls the from function,
// the error type received is converted into the error type defined in the return type of the current function.
//     }
// }

// even shorter
//
// #![allow(unused)]
// fn main() {
//     use std::fs;
//     use std::io;

//     fn read_username_from_file() -> Result<String, io::Error> {
//         fs::read_to_string("hello.txt") // fs::read_to_string function that opens the file, creates a new String,
//         // reads the contents of the file,
//         // puts the contents into that String, and returns it.
//     }

//     let name = read_username_from_file();
//     println!("{name:?}");
// }

// fn last_char_of_first_line(text: &str) -> Option<char> {
//     text.lines().next()?.chars().last() // ? can be used with options
// }

// fn main() {
//     assert_eq!(
//         last_char_of_first_line("Hello, world\nHow are you today?"),
//         Some('d')
//     );

//     assert_eq!(last_char_of_first_line(""), None);
//     assert_eq!(last_char_of_first_line("\nhi"), None);
// }
// ************************************************************************
// ***** Custom Types for Validation *****
//

// #![allow(unused)]
// fn main() {
//     pub struct Guess {
//         value: i32,
//     }

//     impl Guess {
//         // range check in the new method either panics or creates a new Guess with the value and returns it.
//         // A function that has a parameter or returns only numbers between 1 and 100 could then declare in its signature
//         // that it takes or returns a Guess rather than an i32 and wouldn’t need to do any additional checks in its body.
//         pub fn new(value: i32) -> Guess {
//             if value < 1 || value > 100 {
//                 panic!("Guess value must be between 1 and 100, got {value}.");
//             }

//             Guess { value }
//         }

//         pub fn value(&self) -> i32 {
//             self.value
//         }
//     }
// }

// **************************************************************************
// ***** Generic Types *****
// ***** Structs *****

// struct Point<T, U> {
//     // <- allows for different types for each parameter
//     x: T,
//     y: U,
// }

// fn main() {
//     let both_integer = Point { x: 5, y: 10 };
//     let both_float = Point { x: 1.0, y: 4.0 };
//     let integer_and_float = Point { x: 5, y: 4.0 };
// }

// // ***** enums *****

// enum Option<T> {
//     Some(T),
//     None,
// }
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// ***** Methods *****
//

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// fn main() {
//     let p = Point { x: 5, y: 10 };

//     println!("p.x = {}", p.x());
// }

// ***** Traits *****
//

// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// pub struct SocialPost {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub repost: bool,
// }

// impl Summary for SocialPost {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// fn main() {
//     let post = SocialPost {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         repost: false,
//     };

//     let article = NewsArticle {
//         headline: String::from("Penguins win the Stanley Cup Championship!"),
//         location: String::from("Pittsburgh, PA, USA"),
//         author: String::from("Iceburgh"),
//         content: String::from(
//             "The Pittsburgh Penguins once again are the best \
//              hockey team in the NHL.",
//         ),
//     };

//     println!("New article available! {}", article.summarize()); // summarize is used depending on post type
//     println!("1 new post: {}", post.summarize()); // summarize is used depending on post type
// }

// // ***** Traits as parameters (impl Traits) *****
// //
// pub fn notify(item: &impl Summary) {
//     // <- accepts any type that implements Summary
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify2<T: Summary>(item1: &T, item2: &T) {
//     // <- accepts any type that implements Summary, but constrains the parameters to the same type
//     println!("Breaking news! {}", item1.summarize());
//     println!("Further news! {}", item2.summarize());
// }
// pub fn notify3(item: &(impl Summary + Display)) {} // must implement both
// pub fn notify<T: Summary + Display>(item: &T) {} // same

// // ***** Traitbounds with where *****
// //

// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {}

// ***** Returning Types That Implement Traits *****

// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// pub struct SocialPost {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub repost: bool,
// }

// impl Summary for SocialPost {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// fn returns_summarizable() -> impl Summary {
//     SocialPost {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         repost: false,
//     } // while the compiler doesn't need to know the return type, only one possible type may be returned
// }

// ***** Using Trait Bounds to Conditionally Implement Methods *****

// use std::fmt::Display;

// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }
// }

// impl<T: Display + PartialOrd> Pair<T> { // the impl of this method is conditionally bound to T implementing Display and PartialOrd
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//         } else {
//             println!("The largest member is y = {}", self.y);
//         }
//     }
// }
//
// ***** blanked implementations *****
//
// impl<T: Display> ToString for T {
//     // --snip--
//     // Because the standard library has this blanket implementation, we can call the to_string method defined by the ToString trait
//     // on any type that implements the Display trait. For example, we can turn integers into their corresponding String values like
//     // this because integers implement Display
// }
// ***************************************************************************************
// ***** Generic Lifetimes in Functions *****

// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {result}");
// }

// // Lifetime annotations don’t change how long any of the references live.
// // Rather, they describe the relationships of the lifetimes of multiple references to each other
// // without affecting the lifetimes.
// // Lifetime annotations have a slightly unusual syntax:
// // the names of lifetime parameters must start with an apostrophe (')
// // and are usually all lowercase and very short, like generic types.
// // Most people use the name 'a for the first lifetime annotation.
// // We place lifetime parameter annotations after the & of a reference,
// // using a space to separate the annotation from the reference’s type.
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() { x } else { y } // it doesn't matter which reference we return
//     // as we state that the lifetime will be the same for either
// }
// // When we pass concrete references to longest, the concrete lifetime that is substituted for 'a
// // is the part of the scope of x that overlaps with the scope of y. In other words,
// // the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes
// // of x and y. Because we’ve annotated the returned reference with the same lifetime parameter 'a,
// // the returned reference will also be valid for the length of the smaller of the lifetimes of x and y.

// ***** Lifetime Annotations in Struct Definitions *****

// #[derive(Debug)]

// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().unwrap();
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
//     println!("{i:#?}");
// }
// ***** Lifetime Elision *****

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
//     let my_string = String::from("hello world");

//     // first_word works on slices of `String`s
//     let word = first_word(&my_string[..]);

//     let my_string_literal = "hello world";

//     // first_word works on slices of string literals
//     let word = first_word(&my_string_literal[..]);

//     // Because string literals *are* string slices already,
//     // this works too, without the slice syntax!
//     let word = first_word(my_string_literal);
// }
// fn first_word<'a>(s: &'a str) -> &'a str {...}  // This isn't necessary,
// the compiler recognizes this pattern and omits the demand for lifetime annotations

// ***** Lifetime Annotations in Method Definitions *****

// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// impl<'a> ImportantExcerpt<'a> {
//     fn level(&self) -> i32 {
//         3
//     }
// }

// impl<'a> ImportantExcerpt<'a> {
//     fn announce_and_return_part(&self, announcement: &str) -> &str {
//         println!("Attention please: {announcement}");
//         self.part
//     }
// }

// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().unwrap();
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
// }

// ***** Generic Type Parameters, Trait Bounds, and Lifetimes Together *****
//

// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result =
//         longest_with_an_announcement(string1.as_str(), string2, "Today is someone's birthday!");
//     println!("The longest string is {result}");
// }

// use std::fmt::Display;

// fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
// where
//     T: Display,
// {
//     println!("Announcement! {ann}");
//     if x.len() > y.len() { x } else { y }
// }

// *********************************************************************************
// ***** Tests *****
//
// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn exploration() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// } // test passes

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn exploration() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }

//     #[test]
//     fn another() {
//         panic!("Make this test fail"); // test failes
//     }
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         (self.width > other.width && self.height > other.height)
//             || (self.height > other.width && self.width > other.height)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         assert!(larger.can_hold(&smaller)); // should be true => pass
//     }

//     #[test]
//     fn smaller_cannot_hold_larger() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         assert!(!smaller.can_hold(&larger), "Smaller check failed.");
//     }
// }
//

// pub struct Guess {
//     value: i32,
// }

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 {
//             panic!("Guess value must be greater than or equal to 1, got {value}.");
//         } else if value > 100 {
//             panic!("Guess value must be less than or equal to 100, got {value}.");
//         }

//         Guess { value }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[should_panic(expected = "less than or equal to 100")]
//     fn greater_than_100() {
//         Guess::new(200);
//     }
// }

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() -> Result<(), String> {
//         let result = add(2, 2);

//         if result == 4 {
//             Ok(())
//         } else {
//             Err(String::from("two plus two does not equal four"))
//         }
//     }
// }

// f you don’t want to run the tests in parallel or if you want more fine-grained control
// over the number of threads used, you can send the --test-threads flag and the number
// of threads you want to use to the test binary. Take a look at the following example:

// $ cargo test -- --test-threads=1

// fn prints_and_returns_10(a: i32) -> i32 {
//     println!("I got the value {a}"); // This output is only shown on fail, otherwise we must use --show-output
//     10
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn this_test_will_pass() {
//         let value = prints_and_returns_10(4);
//         assert_eq!(value, 10);
//     }

//     #[test]
//     fn this_test_will_fail() {
//         let value = prints_and_returns_10(8);
//         assert_eq!(value, 5);
//     }
// }

// cargo test name_of_the_test only executes the named test,
// partial name matches execute all matching tests
// we can exclude tests with #[ignore]

// ***** Organizing Tests *****

// ***** Unit tests *****

// pub fn add_two(a: usize) -> usize {
//     internal_adder(a, 2)
// }

// fn internal_adder(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn internal() {
//         let result = internal_adder(2, 2);
//         assert_eq!(result, 4);
//     }
// }

// // ***** Integration test *****

// use adder::add_two;

// #[test]
// fn it_adds_two() {
//     let result = add_two(2);
//     assert_eq!(result, 4);
// }

// // This structure prevents Rust treating the common test module as
// // tests that run and can be used for setup functions
// //
// ├── Cargo.lock
// ├── Cargo.toml
// ├── src
// │   └── lib.rs
// └── tests
//     ├── common
//     │   └── mod.rs
//     └── integration_test.rs

//     use adder::add_two;

//     mod common;

//     #[test]
//     fn it_adds_two() {
//         common::setup();

//         let result = add_two(2);
//         assert_eq!(result, 4);
//     }

// *******************************************************************************
// ***** Closures *****
//

// #[derive(Debug, PartialEq, Copy, Clone)]
// enum ShirtColor {
//     Red,
//     Blue,
// }

// struct Inventory {
//     shirts: Vec<ShirtColor>,
// }

// impl Inventory {
//     fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
//         user_preference.unwrap_or_else(|| self.most_stocked()) // the closure captures the immutable reference to self Inventory instance and
//         // passes it with the code we specify to the unwrap_or_else method.
//         // Functions, on the other hand, are not able to capture their environment in this way.
//     }

//     fn most_stocked(&self) -> ShirtColor {
//         let mut num_red = 0;
//         let mut num_blue = 0;

//         for color in &self.shirts {
//             match color {
//                 ShirtColor::Red => num_red += 1,
//                 ShirtColor::Blue => num_blue += 1,
//             }
//         }
//         if num_red > num_blue {
//             ShirtColor::Red
//         } else {
//             ShirtColor::Blue
//         }
//     }
// }

// fn main() {
//     let store = Inventory {
//         shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
//     };

//     let user_pref1 = Some(ShirtColor::Red);
//     let giveaway1 = store.giveaway(user_pref1);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref1, giveaway1
//     );

//     let user_pref2 = None;
//     let giveaway2 = store.giveaway(user_pref2);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref2, giveaway2
//     );

//     // storing a closure in a variable
//     //     let expensive_closure = |num: u32| -> u32 {
//     //         println!("calculating slowly...");
//     //         thread::sleep(Duration::from_secs(2));
//     //         num
//     //     };
//     //

//     // fn and closure syntax similarities / differences
//     // fn  add_one_v1   (x: u32) -> u32 { x + 1 }
//     // let add_one_v2 = |x: u32| -> u32 { x + 1 };
//     // let add_one_v3 = |x|             { x + 1 };
//     // let add_one_v4 = |x|               x + 1  ;
// }
//
// Closures can capture values from their environment in three ways, which directly map to the three ways
// a function can take a parameter: borrowing immutably, borrowing mutably, and taking ownership.
// The closure will decide which of these to use based on what the body of the function does with the captured values.

// use std::thread;

// fn main() {
//     let list = vec![1, 2, 3];
//     println!("Before defining closure: {list:?}");

//     // If you want to force the closure to take ownership of the values it uses in the environment
//     // even though the body of the closure doesn’t strictly need ownership,
//     // you can use the move keyword before the parameter list.
//     thread::spawn(move || println!("From thread: {list:?}"))
//         .join()
//         .unwrap();
// }
// // Once a closure has captured a reference or captured ownership of a value from the environment
// // where the closure is defined (thus affecting what, if anything, is moved into the closure),
// // the code in the body of the closure defines what happens to the references or values
// // when the closure is evaluated later (thus affecting what, if anything, is moved out of the closure).
// // A closure body can do any of the following: move a captured value out of the closure, mutate the captured value,
// // neither move nor mutate the value, or capture nothing from the environment to begin with.

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let mut list = [
//         Rectangle {
//             width: 10,
//             height: 1,
//         },
//         Rectangle {
//             width: 3,
//             height: 5,
//         },
//         Rectangle {
//             width: 7,
//             height: 12,
//         },
//     ];

//     list.sort_by_key(|r| r.width); // uses a closure, but doesn't capture anything
//     println!("{list:#?}");
// }
// *************************************************************************************************
// ***** Iterators *****
//

// #[test]
// fn iterator_demonstration() {
//     let v1 = vec![1, 2, 3];

//     let mut v1_iter = v1.iter();
//     //Note that we needed to make v1_iter mutable: calling the next method on an iterator changes internal state
//     // that the iterator uses to keep track of where it is in the sequence. In other words, this code consumes,
//     // or uses up, the iterator. Each call to next eats up an item from the iterator. We didn’t need to make v1_iter mutable
//     // when we used a for loop because the loop took ownership of v1_iter and made it mutable behind the scenes.
//     // The iter method produces an iterator over immutable references. If we want to create an iterator
//     // that takes ownership of v1 and returns owned values, we can call into_iter instead of iter.
//     // Similarly, if we want to iterate over mutable references, we can call iter_mut instead of iter.

//     assert_eq!(v1_iter.next(), Some(&1));
//     assert_eq!(v1_iter.next(), Some(&2));
//     assert_eq!(v1_iter.next(), Some(&3));
//     assert_eq!(v1_iter.next(), None);
// }

// // ********************

// fn main() {
//     let v1: Vec<i32> = vec![1, 2, 3];

//     let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // collect consulmes the iterator and collects the resulting values.

//     // You can chain multiple calls to iterator adapters to perform complex actions in a readable way.
//     // But because all iterators are lazy, you have to call one of the consuming adapter methods to get results from calls to iterator adapters.

//     assert_eq!(v2, vec![2, 3, 4]);
// }

// #[derive(PartialEq, Debug)]
// struct Shoe {
//     size: u32,
//     style: String,
// }

// fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
//     shoes.into_iter().filter(|s| s.size == shoe_size).collect()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn filters_by_size() {
//         let shoes = vec![
//             Shoe {
//                 size: 10,
//                 style: String::from("sneaker"),
//             },
//             Shoe {
//                 size: 13,
//                 style: String::from("sandal"),
//             },
//             Shoe {
//                 size: 10,
//                 style: String::from("boot"),
//             },
//         ];

//         let in_my_size = shoes_in_size(shoes, 10);

//         assert_eq!(
//             in_my_size,
//             vec![
//                 Shoe {
//                     size: 10,
//                     style: String::from("sneaker")
//                 },
//                 Shoe {
//                     size: 10,
//                     style: String::from("boot")
//                 },
//             ]
//         );
//     }
// }
// **********************************************************************************************
// ***** Smart Pointers *****
// A pointer is a general concept for a variable that contains an address in memory.
// Smart pointers, on the other hand, are data structures that act like a pointer but also have additional metadata and capabilities.
// Boxes

// fn main() {
//     let b = Box::new(5);
//     println!("b = {b}");
// }

// ***** Deref *****
//
