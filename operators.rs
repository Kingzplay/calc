//fn arith(x: &String, y: &String,ops: String) -> f64 {



//}




use crate::err_handlers::err_parse;


pub fn op_pos(var: &Vec<String>,op: String) -> usize{
    let num = var.iter().position(|x| *x == op).unwrap();
    num
}

pub fn calc(var: &Vec<String>,ops: String) -> f64 {
   // println!("{:?} and {}",var.clone(),op.clone());





    let num = op_pos(&var, ops.clone());
    

    match ops.as_str() {
    "+" => err_parse(var[num-1].clone()) + err_parse(var[num+1].clone()),
    "-" => err_parse(var[num-1].clone()) - err_parse(var[num+1].clone()),
    "*" => err_parse(var[num-1].clone()) * err_parse(var[num+1].clone()),
    "/" => err_parse(var[num-1].clone()) / err_parse(var[num+1].clone()),
    _ => {println!("ERROR calc0 : Something went wrong...\nPlease file an issue on the github repo.");std::process::exit(1)}
}


 //   arith(&v[num-1].clone(),&v[num+1].clone(),op.clone())    
    

}


pub fn rep(var: &mut Vec<String>,ops: String) {
    let num = calc(var,ops.clone());
    let index = op_pos(var, ops);
    std::mem::replace(&mut var[index], num.to_string());
    //let a = &index-1;
    var.remove(index-1);
    var.remove(index);    

}
