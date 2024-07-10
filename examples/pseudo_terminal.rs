// FROM HERE
// https://www.joshmcguigan.com/blog/build-your-own-shell-rust/

use std::io::stdin;
use std::io::stdout;
use std::io::Write;
// use std::process::Command;
fn main() {
    loop {
        // use the `>` character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        print!("> ");
        let _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input = input.trim().to_string();

        // shell exit
        if input == "exit"  { break ; };

        
        println!("{}",input);
    }
}
