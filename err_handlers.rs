use std::process;



pub fn alpha_check(var: &String) -> bool {
    let mut x = true;
    for i in var.chars() {
        if i.is_alphabetic()  {
            x = false;           
            break
            
        }
    }
    x
}


pub fn err_parse(var: String) -> f64 {
        let var: f64 = match var.parse() {
            Ok(k) => k,
            Err(e) => {println!("ERROR: Expected an integer value , found invalid number\nCode error: conv1"); process::exit(1)}

        };
    var    

}



pub fn err_args(args: &Vec<String>,case: i32) {
    //case 1
    if args.len() < 2 && case == 1 {
        println!("ERROR: Not enough arguments.");
        process::exit(1)}
    

    //case 2    
    else if args.len() > 5 && case == 2 {
        println!("ERROR: Too many arguments.");
        process::exit(1)}
    else if args.len() < 5 && case == 2 {
        println!("ERROR: Not enough arguments.");
        process::exit(1)}
    }
