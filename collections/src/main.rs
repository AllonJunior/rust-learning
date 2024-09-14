use  std::collections::HashMap;

fn practice_vector(){
    let mut v: Vec<i32> = vec![1,2,3];
    v.push(4);
    v.push(5);
    v.push(6);
    for i in &v {
        println!("{i} is {:?}", *i);
    }
    let err: Option<&i32> = v.get(3);
    match err{
        Some(err) => println!("err is {:?}", err),
        None => println!("Error happened")
    }
}

#[derive(Debug)]
enum Multitypes {
    Init(i32),
    Name(String),
    Size(f32)
}

fn vector_multityples() {
    let v = vec![
        Multitypes::Init(3),
        Multitypes::Name(String::from("John")),
        Multitypes::Size(10.2)
    ];
    for item in &v {
        println!("{:?}", *item);
    } 
    println!("size of v is {:?}", v.len());
}

// Strings
fn string_practice() {
    let s = String::from("Здравствуйте");
    for a in s.chars() {
        println!("{a}");
    }
    let sub = &s[0..4];
    println!("{:?}", sub);

    for b in s.bytes(){
        println!("{b}");
    }
}

// HashMap
fn hashmap_practice(){
    let mut map = HashMap::new();
    map.insert(String::from("Blue"), 10);
    map.insert(String::from("Yellow"), 20);
    map.insert(String::from("Purple"), 30);
   // let teamname = String::from("Blue");
    println!("{:?}", map.get(&String::from("Blue")).copied().unwrap_or(0));
    for (k, v) in map {
        println!("{k}:{v}");
    }
}
fn hashmap_update() {
    let mut upmap = HashMap::new();
    let name = String::from("Team");
    let value = 32;
    upmap.insert(name, value);
//    println!("{name}, {value}");  // name already moved to Hashmap, ownership transfered.
    println!("before update: {:?}", upmap);
    upmap.insert(String::from("Team"), 20);
    println!("after updated: {:?}", upmap);
    upmap.insert(String::from("Group"), 110);
    println!("when new item inserted: {:?}", upmap);

    // Entry: if a key does exist, keep it, if not, insert a new value
    upmap.entry(String::from("Team")).or_insert(30);
    upmap.entry(String::from("Depart")).or_insert(90);
    println!("after insert with entry: {:?}", upmap);

    // Update the existing value
    let mut wordFrequency = HashMap::new();
    let sent = String::from("如果读一些Java或者相关框架的源码，实际上一定会经常出现invoke方法的调用");
    // for word in sent.split_whitespace() {
    //     let count = wordFrequency.entry(word).or_insert(0);
    //     *count += 1;
    // }
    for word in sent.chars(){
        let count = wordFrequency.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", wordFrequency);

}


fn main() {
    // println!("Hello, world!");
    // practice_vector();
    // vector_multityples();

    // string_practice();
    hashmap_practice();
    hashmap_update();
}
