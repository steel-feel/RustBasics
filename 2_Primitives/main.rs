
use std::fmt;
// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

/*
Activity 1: 
Add the fmt::Display trait to the Matrix struct in the above example,
so that if you switch from printing the debug format {:?} to the display format {},
you see the following output:
( 1.1 1.2 )
( 2.1 2.2 )
*/

impl fmt::Display for Matrix {

   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

      //deconsturct the Matrix
      // let (a1, a2, b1, b2) = self;

      write!(f, "( {0} {1} )\n( {2} {3} )", self.0,self.1,self.2,self.3   )
   }

}

/*
Activity 2: 
Add a transpose function using the reverse function as a template,
which accepts a matrix as an argument, 
and returns a matrix in which two elements have been swapped
*/
fn transpose (matrix: Matrix) -> Matrix {
   //Destructure matrix using let 
  let Matrix(a1,a2,b1,b2) = matrix;

   Matrix(a1,b1,a2,b2)
}


fn main() {
   let m1 = Matrix(1f32,2f32,3f32,4f32);

   println!("Matrix \n{}", m1); 
   println!("Transposed Matrix \n{}", transpose(m1)); 

   //Arrays
       // Fixed-size array (type signature is superfluous)
       let xs: [i32; 5] = [1, 2, 3, 4, 5];

       // All elements can be initialized to the same value
      //  let ys: [i32; 500] = [0; 500];
       
       //Slice, start from index 2 but 4th index is not included its [2,4);
       let slice_xs = &xs[2..4];

       println!("slice values {}", slice_xs[0]);
   
}