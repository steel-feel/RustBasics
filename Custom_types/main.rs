/*

2. Add a function square which takes a Point and a f32 as arguments, and returns a Rectangle with its top left corner on the point, 
and a width and height corresponding to the f32.
*/

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}


fn main(){

    let t_left = Point{x:10f32, y : 20f32};
    let    b_right =  Point{x: 20f32 ,y: 12f32};
     let   rect = Rectangle{ top_left: t_left, bottom_right: b_right };
    
    rect_area(rect);  
    
    let square_point = Point{x:10.0,y:20.6};

    print!("Square is {:?}", square(square_point,3.0))



}
//1. Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring).
fn rect_area(rect: Rectangle) {

    let Rectangle{ top_left: Point{x: min_x,y: max_y }, bottom_right: Point{ x: max_x,y: min_y }   } = rect;

    let length = max_x - min_x;
    let breadth = max_y - min_y;

    println!("Area of rectangle is {}", length*breadth);   

}


fn square(top_left : Point, side: f32) -> Rectangle {
    let Point{ x : top_left_x, y: top_left_y } = top_left;
    
    let rect = Rectangle{  top_left : top_left, bottom_right: Point{ x : top_left_x + side, y: top_left_y - side } };
    return rect;
}