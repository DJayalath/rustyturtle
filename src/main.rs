extern crate minifb;

use minifb::{Key, WindowOptions, Window, KeyRepeat}; // For window
use std::fs; // For file reading
use std::env; // For args
use std::process;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {

    // Create turtle
    let mut turtle = Turtle::new((0, 0), Orientation::NORTH, true, 10, 0x00FFFFFF);

    // Construct pixel buffer
    let mut pixel_buffer = PixelBuffer::new(vec![0; WIDTH * HEIGHT], vec![0; (turtle.size * turtle.size) as usize]);

    // Create window
    let mut window = Window::new("rustyturtle", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Collect file name
    let args: Vec<String> = env::args().collect();

    // Handle case where file not given in args
    if args.len() != 2 {
        println!("File not detected!");
    } else {

        let filename = &args[1];

        if let Err(e) = process_file(filename.clone(), &mut turtle, &mut pixel_buffer) {
            println!("ERROR - {}", e);
            process::exit(1);
        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // Clear turtle indicator pos
        if turtle.pen_down {
            pixel_buffer.draw(&turtle, DrawMode::NORMAL);
        } else {
            pixel_buffer.draw(&turtle, DrawMode::PASSIVE);
        };

        if let Err(e) = process_input(&mut window, &mut turtle) {
            println!("ERROR - {}", e);
        }

        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            turtle.pen_down = !turtle.pen_down;
        }

        pixel_buffer.draw(&turtle, DrawMode::ACTIVE);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&pixel_buffer.full).unwrap();
    }
}

enum Orientation {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

enum DrawMode {
    ACTIVE,
    NORMAL,
    PASSIVE,
}

struct PixelBuffer {
    full: Vec<u32>,
    last: Vec<u32>,
}

impl PixelBuffer {
    fn new(full: Vec<u32>, last: Vec<u32>) -> PixelBuffer {
        PixelBuffer { full, last }
    }

    // TODO: Vector error checking
    fn draw(&mut self, turtle: &Turtle, mode: DrawMode) {

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

struct Turtle {
    pos: (i32, i32),
    direction: Orientation,
    pen_down: bool,
    size: u32,
    colour: u32,
}

impl Turtle {
    fn new(pos: (i32, i32), direction: Orientation, pen_down: bool, size: u32, colour: u32) -> Turtle {
        Turtle { pos, direction, pen_down, size, colour }
    }

    fn displace(&mut self, amt: (i32, i32)) -> Result<(), &'static str> {

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

fn process_file(filename: String, turtle: &mut Turtle, pixel_buffer: &mut PixelBuffer) -> Result<(), String> {

    // Read file if available
    let contents = fs::read_to_string(filename).expect("Failed to read file contents!");

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
                        let msg = format!("Line {}: Unknown instruction for PEN. Use either 'PEN DOWN' or 'PEN UP'", line_num);
                        return Err(msg.clone());
                        },
                };
            },
            "COLOUR" | "COLOR" => {
                turtle.colour = val.parse::<u32>().expect("Failed to parse colour! Format RGB e.g. 255255255");
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

                let repeats = val.parse::<u32>().expect("Failed to parse repeats! Format as a positive integer.");

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

fn process_input(window: &mut minifb::Window, turtle: &mut Turtle) -> Result<(), &'static str> {

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
        println!("Application error: {}", e);
    }

    Ok(())
}