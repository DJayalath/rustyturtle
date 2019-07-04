# rustyturtle
Turtle graphics for Rust

## Live Controls
WASD/Arrow keys = Move turtle

Space = Toggle pen

## Programming rustyturtle
Insert instructions line by line into a text file of your choice using the following format:

DIRECTION REPEATS e.g. SOUTH 50


Supports NORTH, SOUTH, EAST and WEST directions. You can also use the special instructions "PEN DOWN" and "PEN UP". Diagonals such as SOUTHWEST are supported.


If compiling, use cargo run filename.txt to use this programmed file.


The interpreter ignores all code after the second space so you can use this to make comments!