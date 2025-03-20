pub trait printFun {
    fn fun(&self) -> String;
}
pub struct firstStruct {
    pub firstField : String,
    pub secondField : i32,
    pub thirdField : i32
}

impl printFun for firstStruct {
    fn fun(&self) -> String {
        format!("this is firstStruct output: {} - {}, {}", self.firstField, self.secondField, self.thirdField)
    }
}


pub fn traitsExample (){
    let obj1 = firstStruct{
        firstField : String::from("Hey I am the first feild "),
        secondField : 2,
        thirdField : 3
    };

    println!("this the result of testing   value of fun  = {}",obj1.fun());
}