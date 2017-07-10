fn main() {
    println!(
        "{:?}",
        brain_luck(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.", vec![3, 2])
    );
}

fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let mut token = 0;
    let mut ptr = 0;
    let mut io_ptr = 0;

    let mut jmp = false;

    let mut jumps = vec![];
    let mut stack: Vec<u8> = vec![0];
    let mut output: Vec<u8> = vec![];

    loop {
        if let Some(op) = code.chars().nth(token) {

            if jmp {
                println!("-> {}:{}", token, op);
                if op == ']' {
                    jmp = false;
                }

                token += 1;
                continue;
            }

            println!("{}:{}", token, op);

            match op {
                '[' => {
                    if stack[ptr] == 0 {
                        jumps.pop();
                        jmp = true;
                    } else {
                        jumps.push(token);
                    }
                }
                ']' => {
                    if stack[ptr] != 0 {
                        token = jumps[jumps.len() - 1] - 1;
                    } else {
                        jumps.pop();
                    }
                }
                '.' => {
                    output.push(stack[ptr]);
                    println!("PUT ->> {}", stack[ptr]);
                }
                ',' => {
                    println!("GET ->> {}", input[io_ptr]);

                    stack[ptr] = input[io_ptr];
                    io_ptr += 1;
                }
                '+' => {
                    if stack[ptr] != 255 {
                        stack[ptr] += 1;
                    } else {
                        stack[ptr] = 0;
                    }
                }
                '-' => {
                    if stack[ptr] != 0 {
                        stack[ptr] -= 1;
                    } else {
                        stack[ptr] = 255;
                    }
                }
                '>' => {
                    ptr += 1;

                    if stack.len() <= ptr {
                        stack.push(0);
                    }
                }
                '<' => {
                    ptr -= 1;
                }
                _ => {
                    break;
                }
            }

            token += 1;
        } else {
            break;
        }
    }

    output
}

#[test]
fn subtract() {
    assert_eq!(brain_luck(",>,[-<->]<.", vec![4, 3]),  vec![1]);
}
#[test]
fn ready_until_zero() {
    assert_eq!(brain_luck(",[.[-],]", vec![1, 1, 1, 0]),  vec![1, 1, 1]);
}
#[test]
fn ready_until_255() {
    assert_eq!(brain_luck(",+[-.,+]", vec![1, 1, 1, 255]),  vec![1, 1, 1]);
}