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
                    let read_result = read(&trimmed_line);
                    match read_result {
                        Ok(read_result) => println!("{}", print(eval(read_result))),
                        Err(err_msg) => println!("{}", err_msg),
                    }
                }
            }
            Err(err) => {
                println!("Failed to read input, {}", err);
                break;
            }
        }
    }
}

fn read(s: &str) -> Result<MalType, String> {
    let rdr = reader::read_str(s);
    reader::read_form(&rdr).map(|x| x.0)
}

fn eval(s: MalType) -> MalType {
    s
}

fn print(mal: MalType) -> String {
    printer::pr_str(&mal)
}
