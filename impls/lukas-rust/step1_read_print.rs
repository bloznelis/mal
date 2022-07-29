mod printer;
mod reader;
mod types;

use crate::types::MalType;

fn main() {
    loop {
        println!("user> ");
        let mut line = String::new();
        let res = std::io::stdin().read_line(&mut line);

        match res {
            Ok(bytes) => {
                if bytes == 0 {
                    break;
                } else {
                    let trimmed_line = line.trim_end();
                    // let tokenized = reader::read_str(&trimmed_line);
                    // let (form, _) = reader::read_form(&tokenized);
                    // println!("{:?}", form);
                    let result: String = print(eval(read(&trimmed_line)));
                    println!("{}", result);
                }
            }
            Err(err) => {
                println!("Failed to read input, {}", err);
                break;
            }
        }
    }
}

fn read(s: &str) -> MalType {
    let reader = reader::read_str(s);
    reader::read_form(&reader).0
}

fn eval(s: MalType) -> MalType {
    s
}

fn print(mal: MalType) -> String {
    printer::pr_str(&mal)
}
