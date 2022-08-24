use std::io;
fn main() 
{
    let mut polish_notation = String::new();
    io::stdin().read_line(&mut polish_notation).expect("Failed to read polish notation!");
    let mut operands: Vec<i8> = Vec::new();
    let string_arr = polish_notation.split_whitespace();
    for item in string_arr
    {
        match item.parse()
        {
            Ok(num) => operands.push(num),
            Err(_) => {
                let op2:i8;
                let op1:i8; 
                match operands.pop()
                {
                    Some(num) => op2 = num,
                    None => 
                    {
                        println!("Invalid polish notation!");
                        break;
                    }
                };
                match operands.pop()
                {
                    Some(num) => op1 = num,
                    None =>
                    {
                        println!("Invalid polish notation!");
                        break;
                    }
                };
                match item 
                {
                    "+" => operands.push(op1 + op2),
                    "-" => operands.push(op1 - op2),
                    "*" => operands.push(op1 * op2),
                    "/" => operands.push(op1 / op2),
                    "%" => operands.push(op1 % op2),
                    &_ => break
                };
            }
        };
    };
    println!();
    println!("{}", operands.pop().unwrap());
}