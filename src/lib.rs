extern crate hex;

use std::fs; // For file reading
use minifb::Key; // For window

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

pub enum Orientation {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

pub enum DrawMode {
    ACTIVE,
    NORMAL,
    PASSIVE,
}

pub struct PixelBuffer {
    pub full: Vec<u32>,
    pub last: Vec<u32>,
}

impl PixelBuffer {
    pub fn new(full: Vec<u32>, last: Vec<u32>) -> PixelBuffer {
        PixelBuffer { full, last }
    }

    pub fn draw(&mut self, turtle: &Turtle, mode: DrawMode) {

        let mut indexer = turtle.pos.1 * WIDTH as i32 + turtle.pos.0;
        let mut counter = 0;

        for _ in 0..turtle.size {
            for j in 0..turtle.size { 
                self.full[indexer as usize + j as usize] = 
                    match mode {
                        DrawMode::ACTIVE => {
                            self.last[counter] = self.full[indexer as usize + j as usize];
                            0x0000FF00
                        },
                        DrawMode::NORMAL => {
                            self.last[counter] = self.full[indexer as usize + j as usize];
                            turtle.colour
                        },
                        DrawMode::PASSIVE => self.last[counter],
                    };
                counter += 1;
            }
            indexer += WIDTH as i32;
        }
    }
}

pub struct Turtle {
    pub pos: (i32, i32),
    pub direction: Orientation,
    pub pen_down: bool,
    pub size: u32,
    pub colour: u32,
}

impl Turtle {
    pub fn new(pos: (i32, i32), direction: Orientation, pen_down: bool, size: u32, colour: u32) -> Turtle {
        Turtle { pos, direction, pen_down, size, colour }
    }

    pub fn displace(&mut self, amt: (i32, i32)) -> Result<(), &'static str> {

        if self.pos.0 + amt.0 < 0 ||
            self.pos.1 + amt.1 < 0 ||
            self.pos.0 + amt.0 + self.size as i32 >= WIDTH as i32 ||
            self.pos.1 + amt.1 + self.size as i32 >= HEIGHT as i32 {
            return Err("Requested movement outside range!");
        }

        self.pos.0 += amt.0;
        self.pos.1 += amt.1;

        Ok(())
    }
}

pub fn process_file(filename: String, turtle: &mut Turtle, pixel_buffer: &mut PixelBuffer) -> Result<(), String> {

    // Read file if available
    let contents = fs::read_to_string(filename);
    let contents = match contents {
        Ok(file) => file,
        Err(e) => {
            let msg = format!("IO Error: {}", e);
            return Err(msg.clone());
        }
    };

    let mut line_num = 1;

    // Run instructions from file
    for instruction in contents.lines() {

        let command: Vec<&str> = instruction.split(" ").collect();
        let (query, val) = (command[0], command[1]);

        match query {
            "PEN" => {
                turtle.pen_down = match val {
                    "UP" => false,
                    "DOWN" => true,
                    _ => {
                        let msg = format!("Line {}: unknown instruction for PEN. Use either 'PEN DOWN' or 'PEN UP'", line_num);
                        return Err(msg.clone());
                        },
                };
            },
            "COLOUR" | "COLOR" => {
                let colour = hex::decode(val);
                turtle.colour = match colour {
                    Ok(col) => {

                        let mut col_arr: [u8; 4] = [0; 4];
                        for i in 1..4 {
                            col_arr[i] = col[i - 1];
                        }

                        unsafe { std::mem::transmute::<[u8; 4], u32>(col_arr) }.to_be()
                    }
                    Err(e) => {
                        let msg = format!("Line {}: {}", line_num, e);
                        return Err(msg.clone());
                    }
                }
            },
            _ => {

                let mut mov_amt: (i32, i32) = (0, 0);

                if query.contains("NORTH") {
                    mov_amt.1 -= 1;
                }
                if query.contains("SOUTH") {
                    mov_amt.1 += 1;
                }
                if query.contains("EAST") {
                    mov_amt.0 += 1;
                }
                if query.contains("WEST") {
                    mov_amt.0 -= 1;
                }

                let repeats = val.parse::<u32>();
                let repeats = match repeats {
                    Ok(num_repeats) => num_repeats,
                    Err(e) => {
                        let msg = format!("Line {}: {}", line_num, e);
                        return Err(msg.clone());
                    }
                };

                if repeats <= 0 {
                    let msg = format!("Line {}: repeats must be a positive integer", line_num);
                    return Err(msg.clone());
                }

                for _ in 0..repeats {

                    if let Err(e) = turtle.displace(mov_amt) {
                        let msg = format!("Line {}: {}", line_num, e);
                        return Err(msg.clone());
                    }

                    if turtle.pen_down {pixel_buffer.draw(&turtle, DrawMode::NORMAL)};
                }
            }
        }

        line_num += 1;

    }

    Ok(())
}

pub fn process_input(window: &mut minifb::Window, turtle: &mut Turtle) -> Result<(), &'static str> {

    let mut mov_amt: (i32, i32) = (0, 0);

    if window.is_key_down(Key::W) || window.is_key_down(Key::Up) {
        mov_amt.1 -= 1;
        turtle.direction = Orientation::NORTH;
    } else if window.is_key_down(Key::S) || window.is_key_down(Key::Down) {
        mov_amt.1 += 1;
        turtle.direction = Orientation::SOUTH;
    }
    if window.is_key_down(Key::A) || window.is_key_down(Key::Left) {
        mov_amt.0 -= 1;
        turtle.direction = Orientation::WEST;
    } else if window.is_key_down(Key::D) || window.is_key_down(Key::Right) {
        mov_amt.0 += 1;
        turtle.direction = Orientation::EAST;
    }

    if let Err(e) = turtle.displace(mov_amt) {
        println!("ERROR - {}", e);
    }

    Ok(())
}