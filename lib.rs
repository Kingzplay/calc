mod err_handlers;
mod split_keep;
mod operators;


use err_handlers::{err_parse, alpha_check};
use split_keep::split_keep;
use operators::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a() {   
    let mut a = vec![String::from("5"),String::from("*"),String::from("7")];
    rep(&mut a, String::from("*"), String::from("35"));
    println!("A is equal to {:?}",a);
    assert_eq!(a,vec![String::from("35")])
    }
}





pub fn matcher(exp_original: String) -> f64 {
    let mut exp = split_keep(&exp_original);

    if !alpha_check(&exp_original) {
        println!("ERROR");
        9.7 }
    

    else if exp.len() == 3 {
        operators::calc(&exp,exp[1].clone()) }


    else if exp.len() > 3 {

        if exp.contains(&String::from("*")) ||  exp.contains(&String::from("/")) {
            for (i,o) in exp.clone().into_iter().enumerate() {
                if o == String::from("*") {
                    rep(&mut exp,o.clone()) 
                }

                 else if o == String::from("/") {
                    rep(&mut exp,o.clone()) 
                }
            }
        
        }     
            for (i,o) in exp.clone().into_iter().enumerate() {
                if o == String::from("+") {
                    rep(&mut exp,o.clone()) 
                }

                 else if o == String::from("-") {
                    rep(&mut exp,o.clone()) 
                }
            } 
    err_parse(exp[0].clone())
    }


    else { 
        println!("ERROR");
        9.7}
}

