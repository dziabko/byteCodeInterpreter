#[derive(Debug)]
struct ByteCode {
    instn: String,
    val: Option<i32>
}

fn main() {
    
    // // LOOPS (ATTEMPT)
    // // z = 4; while (z < 5) {z = z + 1} y = 1; (LOAD_VAL 4; WRITE_VAR 'Z'; LOAD_VAR 'Z'; LOAD_VAL 5; SUB; BRANCH_MORE_THAN_ZERO @@@@@@; LOAD_VAR 'Z'; LOAD_VAL 1; ADD; WRITE_VAR 'Z'; LOAD_VAL 1; WRITE_VAR 'Y')
    // let mut bc_array = vec![
    //     ByteCode { instn: "LOAD_VAL".to_string(), val: Some(4)},
    //     ByteCode { instn: "WRITE_VAR".to_string(), val: None},
    //     ByteCode { instn: "READ_VAR".to_string(), val: None},
    //     ByteCode { instn: "LOAD_VAL".to_string(), val: Some(5)},
    //     ByteCode { instn: "BRANCH_LESSTHAN".to_string(), val: None},
    //     ByteCode { instn: "READ_VAR".to_string(), val: None},
    //     ByteCode { instn: "LOAD_VAL".to_string(), val: Some(1)},
    //     ByteCode { instn: "ADD".to_string(), val: None},
    //     ByteCode { instn: "WRITE_VAR".to_string(), val: None},
    //     ByteCode { instn: "LOAD_VAL".to_string(), val: Some(1)},
    //     ByteCode { instn: "WRITE_VAR".to_string(), val: None}
    // ];

    // // println!("BC_ARRAY: {:?}", bc_array[2]);
    // let mut stack2 = vec![];
    // let mut pc = 0;
    // let mut empty_var: i32 = 0; 
    // interpr(&mut stack2, &mut bc_array, &mut empty_var, &mut pc);
    // interpr(&mut stack2, &mut bc_array, &mut empty_var, &mut pc);
    // interpr(&mut stack2, &mut bc_array, &mut empty_var, &mut pc);
    // interpr(&mut stack2, &mut bc_array, &mut empty_var, &mut pc);
    // interpr(&mut stack2, &mut bc_array, &mut empty_var, &mut pc);
    // interpr(&mut stack2, &mut bc_array, &mut empty_var, &mut pc);
    // interpr(&mut stack2, &mut bc_array, &mut empty_var, &mut pc);
    // interpr(&mut stack2, &mut bc_array, &mut empty_var, &mut pc);
    // interpr(&mut stack2, &mut bc_array, &mut empty_var, &mut pc);
    // interpr(&mut stack2, &mut bc_array, &mut empty_var, &mut pc);
    // interpr(&mut stack2, &mut bc_array, &mut empty_var, &mut pc);



    
}


fn interpr(stack: &mut Vec<i32>, byte_code: &mut ByteCode, var: &mut i32) {
    // Interpreter takes in 1 byte code at a time, and executes that byteocde on a stack
    // Bytecode can be thrown into an array, and have the interpreter loop over each bytecode until completed execution
    // An array of bytecode will require a program_counter to keep track of current execution, but will also allow the interpreter to branch to different parts of the bytecode allowing for conditions, and loops 
    if byte_code.instn == "LOAD_VAL".to_string() {
        println!("LOADING VAL");
        match byte_code.val {
            Some(val) => stack.push(val),
            None => println!("Has no value"),
        }
    }
    else if byte_code.instn == "READ_VAR".to_string() {
        println!("READING VAR {:?}", *var);
        stack.push(*var);
    }
    else if byte_code.instn == "WRITE_VAR".to_string() {
        println!("WRITING VAR");
        match stack.pop() {
            Some(val) => *var = val,
            None => println!("Has no value"),
        }
        // *var = stack.pop();
    }
    else if byte_code.instn == "ADD".to_string() {
        println!("ADD");
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(a + b);
    }
    else if byte_code.instn == "SUB".to_string() {
        println!("SUB");
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(b - a);
    }
    else if byte_code.instn == "MUL".to_string() {
        println!("MUL");
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(a * b);
    }
    // else if byte_code.instn == "BRANCH_LESSTHAN".to_string() {
    //     println!("BRANCH_LESSTHAN");
    //     let a = stack.pop().unwrap();
    //     let b = stack.pop().unwrap();
    //     if b < a {
    //         match byte_code.val {
    //             Some(val) => *pc = val,
    //             None => println!("Has no value"),
    //         }
    //     }else {
    //         // Don't branch if condition not met
    //         *pc = *pc + 1;
    //     }
    // }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assgn_add() {    
        // Testing loading stack, writing/reading variable, and adding
        println!("ASSIGNMENT, and basic arithmetic test 'a=7; b = a + 9; in bytecode");
        let mut stack2 = vec![];
        let mut bc1 = ByteCode {
            instn: "LOAD_VAL".to_string(),
            val: Some(7)
        };
        let mut empty_var: i32 = 4; 
        interpr(&mut stack2, &mut bc1, &mut empty_var);
        println!("Stack is {:?}", stack2);
        
        
        
        
        let mut var_a: i32 = -1;
        let mut bc2 = ByteCode {
            instn: "WRITE_VAR".to_string(),
            val: None,
        };
        interpr(&mut stack2, &mut bc2, &mut var_a);
        println!("var_a is {:?}", var_a);
        
        
        
        let mut bc3 = ByteCode {
            instn: "READ_VAR".to_string(),
            val: None,
        };
        interpr(&mut stack2, &mut bc3, &mut var_a);
        println!("Stack is {:?}", stack2);
        
        let mut bc4 = ByteCode {
            instn: "LOAD_VAL".to_string(),
            val: Some(9)
        };
        let mut empty_var: i32 = 4; 
        interpr(&mut stack2, &mut bc4, &mut empty_var);
        println!("Stack is {:?}", stack2);
        
        
        
        let mut var_b: i32 = -1;
        let mut bc5 = ByteCode {
            instn: "ADD".to_string(),
            val: None,
        };
        interpr(&mut stack2, &mut bc5, &mut var_b);
        println!("ADD: Stack is {:?}", stack2);
        
        
        
        
        let mut bc6 = ByteCode {
            instn: "WRITE_VAR".to_string(),
            val: None
        };
        interpr(&mut stack2, &mut bc6, &mut var_b);
        println!("var_b is {:?}\n", var_b);

        assert_eq!(var_a, 7);
        assert_eq!(var_b, 16);        
    }

    #[test]
    fn test_sub() {
        // SUBTRACTION (a = 4 - 1)
        let mut stack2 = vec![];
        let mut empty_var: i32 = 0; 
        
        let mut bc7 = ByteCode {
            instn: "LOAD_VAL".to_string(),
            val: Some(4)
        };
        interpr(&mut stack2, &mut bc7, &mut empty_var);
        println!("Stack is {:?}", stack2);
        
        let mut bc8 = ByteCode {
            instn: "LOAD_VAL".to_string(),
            val: Some(1)
        };
        let mut empty_var: i32 = 4; 
        interpr(&mut stack2, &mut bc8, &mut empty_var);
        println!("Stack is {:?}", stack2);
        
        let mut var_a: i32 = -1;
        let mut bc5 = ByteCode {
            instn: "SUB".to_string(),
            val: None,
        };
        interpr(&mut stack2, &mut bc5, &mut empty_var);
        println!("SUB: Stack is {:?}", stack2);
        
            let mut bc6 = ByteCode {
            instn: "WRITE_VAR".to_string(),
            val: None
        };
        interpr(&mut stack2, &mut bc6, &mut var_a);
        println!("var_a is {:?}\n", var_a);

        assert_eq!(var_a, 3);

    }

    #[test]
    fn test_mul() {
        // MULTIPLY (3 * 5)

        let mut stack2 = vec![];
        let mut empty_var: i32 = 0; 
    
        let mut bc7 = ByteCode {
            instn: "LOAD_VAL".to_string(),
            val: Some(3)
        };
        interpr(&mut stack2, &mut bc7, &mut empty_var);
        println!("Stack is {:?}", stack2);
        
        let mut bc8 = ByteCode {
            instn: "LOAD_VAL".to_string(),
            val: Some(5)
        };
        let mut empty_var: i32 = 4; 
        interpr(&mut stack2, &mut bc8, &mut empty_var);
        println!("Stack is {:?}", stack2);
        
        let mut var_a: i32 = -1;
        let mut bc5 = ByteCode {
            instn: "MUL".to_string(),
            val: None,
        };
        interpr(&mut stack2, &mut bc5, &mut empty_var);
        println!("SUB: Stack is {:?}", stack2);
        
        let mut bc9 = ByteCode {
            instn: "WRITE_VAR".to_string(),
            val: None
        };
        interpr(&mut stack2, &mut bc9, &mut var_a);
        println!("var_a is {:?}", var_a);

        assert_eq!(var_a, 15);
    }
}