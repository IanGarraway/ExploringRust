
enum Shape {triangle, square, pentagon, octagon}

impl Shape{
    fn corners(self) -> i32{
        match self{
        Shape::triangle => 3,
        Shape::square => 4,
        Shape::pentagon=> 5,
        Shape::octagon=>8,
        _ => 0,
        }

    }
}


fn main() {
    let _tri =Shape::triangle;
    let _squ= Shape::square;
    let _pen = Shape::pentagon;
    let _oct = Shape::octagon;

    println!("Triangle has {:?}", _tri.corners());
    println!("Square has {:?}", _squ.corners());
    println!("Pentagon has {:?}", _pen.corners());
    println!("Octagon has {:?}", _oct.corners());

    
}
