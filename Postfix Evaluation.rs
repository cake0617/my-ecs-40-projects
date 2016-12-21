pub enum Operator {
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
}
pub enum Token {
    Operator(Operator),
    Operand(isize),
}



pub fn eval(tokens: &[Token]) -> Option<isize> {
    let mut operator_count : i32 = 0;
    let mut operand_count : i32 = 0;
    
      match tokens[0]
    {
        Token::Operator(ref a) => {return None},
        _ => (),
    }

    for x in tokens.iter() {
        match x {
            &Token::Operator(ref a) => operator_count += 1,
            &Token::Operand(ref b) => operand_count += 1,
        }

    }

    if operand_count - operator_count != 1 {
        return None
    }


    let mut stack : Vec<isize> = vec![];
    
    for x in tokens.iter() {
        match x {
            &Token::Operator(ref a) => { match a {
                &Operator::Add => {
                    if stack.len() == 1 {return None}
                    let x = stack.pop().unwrap() + stack.pop().unwrap();
                    stack.push(x);
                },
                &Operator::Sub =>{if stack.len() == 1 {return None}
                                    let y = stack.pop().unwrap();
                                  let y2= stack.pop().unwrap();
                    stack.push(y2-y);
                },
                &Operator::Mul =>{if stack.len() == 1 {return None}
                let z = stack.pop().unwrap() * stack.pop().unwrap();
                    stack.push(z);    
                },
                
                }
            },
            
            &Token::Operand(ref b) => stack.push(*b),
        }
    }
    return stack.pop();

}


