pub use std::collections::HashMap;
pub fn hashmaps(){
    let mut hm = HashMap::new();

    hm.insert(1, String::from("hello world"));

    for (i,val) in &hm {
        println!("index = {i} ad value = {val}");
    }
    let binding = String::from("none");
    let first = hm.get(&5).unwrap_or(&binding);
    println!("{first:?}");

    let val2 = hm.entry(2).or_insert(binding);
    println!("{val2:?} and");
    println!("{hm:#?}");
}

pub fn wordCount(){
    let text = "Hey, how are you doing !";

    let mut hm = HashMap::new();

    for i in text.split_whitespace() {
        let count = hm.entry(i).or_insert(0);
        *count += 1;
    }

    println!("word count result = {hm:?}");
}