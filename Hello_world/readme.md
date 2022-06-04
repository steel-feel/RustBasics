# Hello World


rustc will produce a hello binary that can be executed. Output binary will be in same folder as the file 

`
rustc <file-name>
`
or by specifying target folder 

`
rustc --out-dir target <file-name>
`

## Display

to customize the output appearance while using _printin!_, need to manually  implement fmt::Display, which uses the {} print marker

**Note** 
> fmt::Display is not implemented for Vec<T> or for any other generic containers. fmt::Debug must then be used for these generic cases.



