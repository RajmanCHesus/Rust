// cargo run spusti project
// cargo new "nazov projektu" vytvorenie projektu + cargo build
// cargo check skontroluje project
// \src rustfrc main.rs zarovnanie suboru


use std::io;
fn main() {
    let mut x: i32 = 4;
    println!("x is : {}", x);
    
    x = x+2;
    println!("x is : {}", x);

    let tup: (i32, bool, char) = (1, true, 's');

    println!("{}", tup.1);

    let mut input: String = String::new();

    io::stdin().read_line(&mut input).expect("faoled to read line");
    println!("{}", input);

    let c: u8 = 255;
    let v: u8 = 1;

    let z:u8 = c+ v;
    println!("{}" ,z);

    


}
