# rustyturtle
Turtle graphics for Rust

## Live Controls
WASD/Arrow keys = Move turtle


Space = Toggle pen

## Programming rustyturtle
Insert instructions line by line into a text file of your choice using the following format:


INSTRUCTION OPTION e.g. SOUTH 50

### List of commands (Instruction, Option)
- NORTH/EAST/SOUTH/WEST, repeats (0 --> inf.)
- PEN, UP/DOWN
- COLOUR/COLOR, RGB (Decimal 24-bit e.g. COLOUR 000255000)


### Additional notes 
- Diagonals also supported e.g. NORTHEAST 20
- The COLOUR command is experimental and not quite working correctly at the moment. If no colour specified, it defaults to white.
- If compiling, use 'cargo run <filename>' to use your programmed file.
- The interpreter ignores all code after the second space so you can use this to make comments!


NOTE: I have tried to ensure the interpreter provides helpful error messages if it fails to translate your instructions. If it doesn't, please 
let me know so I can fix this in the future.