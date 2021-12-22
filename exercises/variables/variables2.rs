// variables2.rs
// Make me compile! Execute the command `rustlings hint variables2` if you want a hint :)

//Data Types
//https://doc.rust-lang.org/book/ch03-02-data-types.html

fn main() {
    let x = 10; //infer the type
    //let x: i32 = 11; //explicitly say the type
    //let x: char = 'A'; //explicitly say a type different from 10 which causes a 'mismatched' types error
    if x == 10 {
        println!("Ten!");
    } else {
        println!("Not ten!");
    }
}
