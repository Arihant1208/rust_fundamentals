#[derive(Debug)]
struct Point<T> {
    x : T,
    y : T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
         &self.x
    }

}
pub fn genreicsImpl () {
    let mut obj = Point{
        x: 20,
        y: 49
    };

    let valx = obj.x();

    println!("this is point = {obj:?} and this is  the value of x = {valx}")
}


#[derive(Debug)]
struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(&self, other: &Point2<X2, Y2>) -> Point2<X1, Y2> 
    where
        X1: Clone, // Ensure X1 can be cloned
        Y2: Clone, // Ensure Y2 can be cloned
    {
        Point2 {
            x: self.x.clone(), // Clone X1
            y: other.y.clone(), // Clone Y2
        }
    }
}

pub fn genericsImpl_2(){
    let obj = Point2{x:1.89,y:20};

    let obj2 = Point2{x:1.89,y:2.19};

    let finalObj = obj.mixup(&obj2);

    println!("obj1 = {obj:?} , obj2 = {obj2:?} and final result = {finalObj:?}")
}

//////////////// NEED TO RESOLVE THIS ERROR !!!!!!!!!! ///////////////////////////////////////////////////////