extern crate regex;
use regex::Regex;
use std::io::{self, Write};
use std::process;

const COLUMN_NAME_SIZE: usize = 32;
const COLUMN_EMAIL_SIZE: usize = 255;
struct InputBuffer {
    buffer: Option<Box<String>>,
    buffer_length: usize,
    input_length: usize,
}
struct Statement {
    stype: StatementType,
    row_to_insert: Option<Row>,
}
struct Row {
    id: u32,
    username: [char; COLUMN_NAME_SIZE],
    email: [char; COLUMN_EMAIL_SIZE],
}
enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}
enum PrepareResult {
    PrepareSuccess,
    PrepareSyntaxError,
    PrepareUnrecognizedStatement,
}
enum StatementType {
    StatementInsert,
    StatementSelect,
    StatementUnknown,
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
    fn do_meta_command(&self) -> MetaCommandResult {
        if let Some(ref s) = self.buffer {
            if s.as_str() == ".exit" {
                process::exit(0);
            } else {
                return MetaCommandResult::MetaCommandUnrecognizedCommand;
            }
        } else {
            return MetaCommandResult::MetaCommandUnrecognizedCommand;
        }
    }
    fn prepare_statement(&self, statement: &mut Statement) -> PrepareResult {
        let ref buffer = self.buffer.clone().unwrap();
        if buffer.starts_with("insert") {
            let reg = Regex::new(r"^insert \d \S \S$").unwrap();
            if !reg.is_match(buffer) {
                statement.stype = StatementType::StatementUnknown;
                return PrepareResult::PrepareSyntaxError;
            }
            statement.stype = StatementType::StatementInsert;
            return PrepareResult::PrepareSuccess;
        }
        if buffer.starts_with("select") {
            let reg = Regex::new(r"^select \d \S \S$").unwrap();
            if !reg.is_match(buffer) {
                statement.stype = StatementType::StatementUnknown;
                return PrepareResult::PrepareSyntaxError;
            }
            statement.stype = StatementType::StatementSelect;
            return PrepareResult::PrepareSuccess;
        }
        return PrepareResult::PrepareUnrecognizedStatement;
    }
    fn excute_statement(&self, statement: &Statement) {
        match statement.stype {
            StatementType::StatementSelect => println!("This is where we would do an select."),
            StatementType::StatementInsert => println!("This is where we would do a insert."),
            StatementType::StatementUnknown => println!("This is unknown command."),
        }
    }
}
fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}
fn main() {
    let ref mut input_buffer = InputBuffer::new_input_buffer();
    loop {
        print_prompt();
        input_buffer.read_input();
        if let Some(ref s) = input_buffer.buffer {
            if s.starts_with(".") {
                match input_buffer.do_meta_command() {
                    MetaCommandResult::MetaCommandSuccess => {
                        continue;
                    }
                    MetaCommandResult::MetaCommandUnrecognizedCommand => {
                        println!("Unrecognized command '{}'", s);
                        continue;
                    }
                }
            }
        }
        let mut statement: Statement = Statement {
            stype: StatementType::StatementUnknown,
            row_to_insert: None,
        };
        match input_buffer.prepare_statement(&mut statement) {
            PrepareResult::PrepareSuccess => {}
            PrepareResult::PrepareUnrecognizedStatement => {
                let mut buffer = input_buffer.buffer.clone();
                println!("Unrecognized keyword at start of '{}'", buffer.unwrap())
            }
            PrepareResult::PrepareSyntaxError => {
                let mut buffer = input_buffer.buffer.clone();
                println!("Syntax Error: '{}'", buffer.unwrap())
            }
        }
        input_buffer.excute_statement(&statement);
        println!("Executed.");
    }
}
