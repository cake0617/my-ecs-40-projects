#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
    // `/`
    Div,
}

#[derive(Debug, PartialEq)]
pub enum InfixToken {
    Operator(Operator),
    Operand(isize),
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq)]
pub enum PostfixToken {
    Operator(Operator),
    Operand(isize),
}

/// Transforms an infix expression to a postfix expression.
///
/// If the infix expression is valid, outputs `Some(_)`;
/// otherwise, outputs `None`.
pub fn infix_to_postfix(tokens: &[InfixToken]) -> Option<Vec<PostfixToken>> {
    //checking if it is valid or invalid.

    let check = tokens.len();
    if check == 0 {
        return None
    }

    match tokens[0]
        {
            InfixToken::Operator(a) => {return None},
            _ => (),
        }

    let length = tokens.len() - 1;

    match tokens[length]
        {
            InfixToken::Operator(a) => {return None},
            _ => (),
        }

    for i in 0..length{
        if tokens[i] == InfixToken::RightParen{
            match tokens[i+1]
                {
                    InfixToken::Operand(a) => return None,
                    _ => (),
                }
        }
        if tokens[i] == InfixToken::LeftParen{
            if i >=1{
                match tokens[i-1]
                    {
                        InfixToken::Operand(a) => return None,
                        _ => (),
                    }
            }
        }
    }

    let mut operator_count : i32 = 0;
    let mut operand_count : i32 = 0;
    let mut leftparen = 0;
    let mut rightparen = 0;

    for x in tokens.iter() {
        match x {
            &InfixToken::Operator(a) => operator_count += 1,
            &InfixToken::Operand(b) => operand_count += 1,
            &InfixToken::LeftParen => leftparen +=1,
            &InfixToken::RightParen => rightparen +=1,
        }
        if leftparen ==0 && rightparen ==1{
            return None
        }
    }

    if operand_count - operator_count != 1 {
        return None
    }

    if leftparen != rightparen {
        return None
    }

    let mut x = 0;
    for x in 0..length{
        match tokens[x]{
            InfixToken::Operator(a) => match tokens[x+1] {
                InfixToken::Operator(c)=>{return None},
                _ => (),
            },
            InfixToken::Operand(b) => match tokens[x+1]{
                InfixToken::Operand(d) => {return None}
                _ => (),
            },
            _ => (),
        }
    }
    //________________________________

    let mut stack = Vec::new();
    let mut answer = Vec::new();

    for x in tokens.iter(){
        match x{
            &InfixToken::Operand(a) => answer.push(PostfixToken::Operand(a)),
            &InfixToken::LeftParen => stack.push(x),
            &InfixToken::RightParen => {
                loop {
                    let mut x = stack.pop();
                    if x == None {
                        break;
                    }

                    match x.unwrap(){
                        &InfixToken::Operator(z)=> answer.push(PostfixToken::Operator(z)),
                        &InfixToken::LeftParen => break,
                        _ => (),
                    }
                }
            }
            &InfixToken::Operator(b) => {
                stack.push(x);

                loop {
                    let one = stack.pop();
                    let two = stack.pop();

                    if two == None {
                        stack.push(one.unwrap());
                        break
                    }

                    if *two.unwrap() == InfixToken::LeftParen{
                        stack.push(two.unwrap());
                        stack.push(one.unwrap());
                        break
                    }

                    if *two.unwrap() == InfixToken::Operator(Operator::Add) || *two.unwrap() == InfixToken::Operator(Operator::Sub)
                        {
                            if *one.unwrap() == InfixToken::Operator(Operator::Add) || *one.unwrap() == InfixToken::Operator(Operator::Sub){
                                stack.push(one.unwrap());
                                match two.unwrap() {
                                    &InfixToken::Operator(z) => answer.push(PostfixToken::Operator(z)),
                                    _ => (),
                                }

                            }else {
                                stack.push(two.unwrap());
                                stack.push(one.unwrap());
                                break
                            }
                        }
                        else {
                            match two.unwrap() {
                                &InfixToken::Operator(z) => answer.push(PostfixToken::Operator(z)),
                                _ => (),
                            }
                            stack.push(one.unwrap());
                        }
                }
            }
        }
    }

    let mut decision = true;
    while decision == true{
        let x = stack.pop();
        if x == None{
            break;
        }

        match x.unwrap(){
            &InfixToken::Operator(a) => answer.push(PostfixToken::Operator(a)),
            _ => (),
        }

    };
Some(answer)
}

