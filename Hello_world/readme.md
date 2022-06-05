# Hello World


rustc will produce a hello binary that can be executed. Output binary will be in same folder as the file 

`
rustc <file-name>
`
or by specifying target folder 

`
rustc --out-dir target <file-name>
`
## Debug

The **fmt::Debug** trait makes this very straightforward. All types can derive (automatically create) the _fmt::Debug_ implementation. use with _{:?}_ .

## Display

To customize the output appearance while using _printin!_, need to manually  implement fmt::Display, which uses the {} print marker

 
> **Note**: fmt::Display is not implemented for Vec<T> or for any other generic containers. fmt::Debug must then be used for these generic cases.

## Quickes

`
vec.iter().enumerate()
`

**iter** is a method that returns each element in a collection and **enumerate** wraps the result of _iter_ and returns each element as part of a tuple instead.The first element of the tuple returned from _enumerate_ is the index, and the second element is a reference to the element. This is a bit more convenient than calculating the index ourselves.
