use calculator::*;
mod uis;
use uis::uis;

fn main() {
loop {  
    let mut args = uis(String::from(">"));
    match args.as_str() {

        "quit" => break,
        "exit" => break,
        _ => match matcher(args.clone()) {
            9.7 => continue,
            _ => println!("{}",matcher(args))
                                      }
                        }

    }

}
    
   

  


