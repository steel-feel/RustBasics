/*

2. Add a function square which takes a Point and a f32 as arguments, and returns a Rectangle with its top left corner on the point, 
and a width and height corresponding to the f32.
*/
#![allow(dead_code)]
// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Node {
    address: String,
    nodes: Vec<Node>
}



enum Colors {
    Red,
    Green,
    Yellow
}


fn tell_a_fruit(color: Colors) -> String {
    
    match color {
        Colors::Red => return "Apple".to_string(),
        Colors::Green => return "Watermelon".to_string(),
        Colors::Yellow => return "Mango".to_string(),
    }
}


// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn create_tree() -> Node {
    
    let mut root = Node {  address:String::from("1"), nodes: Vec::new() };
    let  child1 = Node {  address:String::from("2"), nodes: Vec::new() };
    let  child2 = Node {  address:String::from("3"), nodes: Vec::new() };
    let  child3 = Node {  address:String::from("4"), nodes: Vec::new() };
    root.nodes.push( child1 );
    root.nodes[0].nodes.push(child2);
    root.nodes[0].nodes[0].nodes.push(child3);
  
    println!("Tree's {:?}",root);
    return root;

}

fn check_permission(_address: String, _rootNode: &Node  ) -> bool{
    let curr_node = _rootNode;
   let result:bool = loop { 

        if _address == curr_node.address
            break true;
        
            
    }

  

}


fn main(){
    let root_node = create_tree();

    // check_permission("2");
    // remove_node("2");
    // check_permission("3");
    // let t_left = Point{x:10f32, y : 20f32};
    // let b_right =  Point{x: 20f32 ,y: 12f32};
    // let rect = Rectangle{ top_left: t_left, bottom_right: b_right };
    
    // rect_area(rect);  
    
    // let square_point = Point{x:10.0,y:20.6};

    // println!("Square is {:?}", square(square_point,3.0));

    // println!("Red color fruit is {}", tell_a_fruit(Colors::Red));

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
    
    Rectangle{  top_left : top_left, bottom_right: Point{ x : top_left_x + side, y: top_left_y - side } }
}