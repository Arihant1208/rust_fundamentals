use std::string;

pub fn vectors() {
    let mut v = vec![1,2,3,4,5];
    v.push(6);
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // let not_exist = &v[100];
    // println!("index out of range error : {not_exist}");

    let not_exist: Option<&i32> = v.get(100);
    match not_exist {
        Some(not_exist) => println!("The not_exist element is {not_exist}"),
        None => println!("There is no not_exist element."),
    }
    
    #[derive(Debug)]
    enum AnswerType {
        Int(i32),
        Text(String)
    }

    let vec = vec![AnswerType::Int(5),AnswerType::Text(String::from("heyyy"))];

    for i in &vec {
        println!("{i:?}");
    }

}