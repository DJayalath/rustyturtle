# rustyturtle
Turtle graphics for Rust

## Live Controls
WASD/Arrow keys = Move turtle

Space = Toggle pen

## Programming rustyturtle
Insert instructions line by line into a text file of your choice using the following format:

DIRECTION REPEATS e.g. SOUTH 50


Supports NORTH, SOUTH, EAST and WEST directions. You can also use the special instructions "PEN DOWN" and "PEN UP". Diagonals such as SOUTHWEST are supported.


You can also set colours with the COLOUR command. Specify in the format: COLOUR RGB e.g. COLOUR 000255000 for green or 000255255 for cyan. This feature is experimental and not quite working correctly at the moment. If no colour specified, it defaults to white.


If compiling, use cargo run filename.txt to use this programmed file.


The interpreter ignores all code after the second space so you can use this to make comments!


NOTE: This is a very primitive and basic instruction set, even so, I've been lazy so the interpreter is extremely unforgiving. All instructions must be formatted exactly as expected. See example.txt to see what a typical program should look like.