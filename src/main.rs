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
fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // The method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar"); // <- The push_str method takes a string slice because we don’t necessarily want to take ownership of the parameter.

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // <- note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("{s1}");
    println!("{s}");
    // let h = s1[0]; // <- Rust strings don’t support indexing, because their length is in bytes
    //                      and UTF-8 characters can take more than one byte per letter
    match s1.chars().nth(0) {
        Some(c) => println!("{c}"),
        None => {}
    };

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}
