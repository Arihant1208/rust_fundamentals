#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    breadth: u32,
}
impl Rectangle {
    pub fn area(&self) -> u32 {
        self.length * self.breadth
    }

    pub fn square (size : u32) -> Self {
        Self{
            length : size,
            breadth : size
        }
    }
}

    // let rect = Rectangle{
    //     length : 20,
    //     breadth : 10
    // };

    // let squ = Rectangle::square(10);

    // println!("the area of rectangle {0} * {1} = {2}", rect.length,rect.breadth,rect.area());
    // println!("create a square of len 20 = {squ:#?}");
