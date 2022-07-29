fn main() {
    loop {
        println!("user> ");
        let mut line = String::new();
        // You may remove '\n' using line.trim_end()
        let res = std::io::stdin().read_line(&mut line);

        match res {
            Ok(bytes) => {
                if bytes == 0 {
                    break;
                } else {
                    let result: &str = print(eval(read(&line)));
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

fn read(s: &str) -> &str {
    s
}

fn eval(s: &str) -> &str {
    s
}

fn print(s: &str) -> &str {
    s
}
