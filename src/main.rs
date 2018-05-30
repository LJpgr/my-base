use std::io::{self, Write};
use std::process;
struct InputBuffer {
    buffer: Option<Box<String>>,
    buffer_length: usize,
    input_length: usize,
}
impl InputBuffer {
    fn new_input_buffer() -> Box<InputBuffer> {
        Box::new(InputBuffer {
            buffer: None,
            buffer_length: 0,
            input_length: 0,
        })
    }
    fn read_input(&mut self) {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap_or_else(|e| {
            println!("Could not read from stdin.msg:{}", e);
            process::exit(1);
        });
        let buffer = buffer.replace("\r\n", "").replace("\n", "");
        if buffer.len() > 0 {
            self.buffer_length = buffer.len();
            self.buffer = Some(Box::new(buffer));
        }
    }
}
fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}
fn main() {
    let mut input_buffer = InputBuffer::new_input_buffer();
    loop {
        print_prompt();
        input_buffer.read_input();
        if let Some(_) = input_buffer.buffer {
            process::exit(0)
        } else {
            println!("Unrecognized command '{:?}'.", input_buffer.buffer)
        }
    }
}
