use std::{cmp, io};

fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("c is:{}, z is:{}, emoji is:{}", c, z, heart_eyed_cat);

    let (x,y,z) = (1, 2.5, "Hello world");
    let tup  = (2, 1.2, ", now!");
    let (a, b, c) = tup;

    let tup_add = (x + a, y+b, z.to_owned()+c);
    println!("tup_add is {:?}", tup_add);

    let months = ["Jan.", "Feb.", "Mar.", "Apr.", "May", "Jun.",
                             "Jul.", "Aug.", "Sep.", "Oct.", "Nov.", "Dec."];

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read a line");

    let index : usize = index.trim().parse().expect("Index must be a number!!!");

    println!("The month you selected is: {:?}", months[cmp::min(months.len(), index-1)]);
}