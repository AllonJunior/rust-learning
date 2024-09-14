use std::fmt::{Debug, Display};

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list{
        if item > largest{
            largest = item;
        }
    }

    largest
}


//Trait Bound Syntax
fn largest<T: PartialOrd>(list: &[T]) -> &T{
    let mut largest = &list[0];
    for item in list {
        if item > largest{
            largest = item;
        }
    }
    largest
}

// struct Point<T>{
//     x: T,
//     y: T,
// }
struct Point<T, U>{
    x: T,
    y: U,
}
// Implement struct
impl <T,U> Point<T,U> {
    fn x(&self) -> &T{
        &self.x
    }
    fn y(&self) -> &U{
        &self.y
    }
    fn mix_up<X2, Y2>(self, other: Point<X2, Y2>) -> Point<T, Y2> {
        Point{
            x: self.x,
            y: other.y,
        }
    }
}

// A trait signature definition
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {} ...)", self.summarize_author())
    }
}

// A new struct definition
pub struct  NewArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewArticle{
    fn summarize_author(&self) -> String {
        format!("{}, by {}, ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet{
    fn summarize_author(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Trait as parameter
fn notify(item: &impl Summary) {
    println!("Trait as parameter test: {}", item.summarize());
}

// Multi traits as parameter
fn notify_multi_traits(item: &(impl Summary + Display)){

}
fn notify_multi_traits_generic<T: Summary + Display>(item: &T){

}

// clearer trait bounds with where clauses
fn some_functions<T: Display+Clone, U: Clone+Debug>(t: &T, u: &U) -> i32{
    0
}
// use "where" for clearer
fn some_functions_where<T,U>(t: &T, u: &U) -> i32
where 
    T: Display + Clone,
    U: Clone + Debug,
    {
        0
}

// Returning types that implement Traits
fn returns_summarizable() -> impl Summary {
    Tweet{
        username: String::from("Allen Ren"),
        content: String::from("Rust skills learning and practice, return summarizable implementation"),
        reply: false,
        retweet: false,
    }
}

// Using trait bounds to conditionally implement methods.
struct Pair<T>{
    x: T,
    y: T,
}
impl <T> Pair<T> {
    fn new(x: T, y: T) -> Self{
        Self{x, y }
    }
}

// Lifetime annotations syntax
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len(){
        x
    }else{
        y
    }
}

// Lifetime for struct
struct ImportantExcerpt<'a>{
    part: &'a str,
}
impl <'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32{
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str{
        println!("Attention please: {}", announcement);
        self.part
    }
    
}


// Summary
fn longest_annotation<'a, T>(
    x: &'a str, 
    y: &'a str,
    ann: T,
) -> &'a str 
where 
    T: Display{
    
    println!("Please attention: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    let number_list = vec![12,234,14,25,45,64];
    let largest_number = largest_i32(&number_list);
    println!("The largest number is: {largest_number}");

    let char_list = vec!['a', 'c', 'e', 't', 'y'];
    let largest_char = largest_char(&char_list);
    println!("The largest char is: {largest_char}");

    let largest = largest(&char_list);
    println!("The largest is: {largest}");

    // generic type for Struct
    let p_int = Point{x:5, y:6};
    let p_float = Point{x:5.0, y:6.0};
    let p_if = Point{x:5, y:6.0};
    println!("x is: {:?}, y is: {:?}", p_if.x(), p_if.y());
    
    let p = Point{x:"Hello", y:"world"};
    let new_p = p_if.mix_up(p);
    println!("p.x = {}, p.y = {}", new_p.x(), new_p.y());


    let article = NewArticle{
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Ren Ren"),
        content: String::from("The Pittsburgh Penguins once again are the best \
             hockey team in the NHL."),
    };
    println!("New article available! {}", article.summarize());

    notify(&article);

    let summarizable = returns_summarizable();
    println!("Return summarizable: {}", summarizable.summarize());

    // lifetime annotation 
    let str1 = String::from("str1 is the longest");
    let result;
    {
        let str2 = String::from("str2 shorter");
        result = longest(str1.as_str(), str2.as_str());
       // println!("{}", result);
    }
   //  println!("{}", result);  // here an error happened due to str2 ran out of life time scope.

    // Lifetime with struct
    let novel_str = String::from("I'm Allen, working in Mercedes-benz as an onboard navigation developer...");
    let first_part = novel_str.split(',').next().expect("Could not find a ','");
    let IE = ImportantExcerpt{
        part: first_part,
    };
    println!("{}", IE.part);
    println!("first part is: {}\n the level is: {}", IE.announce_and_return_part("Be careful!"), IE.level());

}
