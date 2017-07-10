fn main() {
    println!("{:?}", brain_luck(",>,< [ > [ >+ >+ << -] >> [- << + >>] <<< -] >>.", vec![8, 5]));
}

fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let mut i = 0;
    let mut io_ptr = 0;
    let mut stack_ptr = 0;

    let mut jmp = false;

    let mut jumps = vec![];
    let mut stack: Vec<u8> = vec![0];
    let mut output: Vec<u8> = vec![];

    loop {
        if let Some(op) = code.chars().nth(i) {
            // jump until
            if jmp {
                if op == ']' {
                    jmp = false;
                }

                i += 1;
                continue;
            }
            
            match op {
                '[' => {
                    // jump wihtout executing between
                    if stack[stack_ptr] == 0 {
                        jmp = true;
                    } else {
                        // set jump target
                        jumps.push(i);
                    }
                }
                ']' => {
                    jmp = false;

                    if stack[stack_ptr] == 0 {
                        // no need to return remove last jump
                        jumps.pop();
                    } else {
                        // return to last jump
                        i = jumps[jumps.len() - 1] - 1;
                        jumps.pop();
                    }
                }
                '.' => {
                    output.push(stack[stack_ptr]);
                }
                ',' => {
                    stack[stack_ptr] = input[io_ptr];
                    io_ptr += 1;
                }
                '+' => {
                    if stack[stack_ptr] != 255 {
                        stack[stack_ptr] += 1;
                    } else {
                        stack[stack_ptr] = 0;
                    }
                }
                '-' => {
                    if stack[stack_ptr] != 0 {
                        stack[stack_ptr] -= 1;
                    } else {
                        stack[stack_ptr] = 255;
                    }
                }
                '>' => {
                    stack_ptr += 1;

                    // extend stack if needed
                    if stack.len() <= stack_ptr {
                        stack.push(0);
                    }
                }
                '<' => {
                    stack_ptr -= 1;
                }
                _ => {
                }
            }

            i += 1;
        } else {
            break;
        }
    }

    output
}

#[test]
fn read_until_zero() {
    assert_eq!(brain_luck(",[.[-],]", vec![1, 1, 1, 0]),  vec![1, 1, 1]);
}
#[test]
fn read_until_255() {
    assert_eq!(brain_luck(",+[-.,+]", vec![1, 1, 1, 255]),  vec![1, 1, 1]);
}
#[test]
fn a() {
    assert_eq!(brain_luck("++++++ [ > ++++++++++ < - ] > +++++ .", vec![]),  vec![65]);
}
#[test]
fn subtract() {
    assert_eq!(brain_luck(",>,[-<->]<.", vec![4, 3]),  vec![1]);
}
#[test]
fn multiply() {
    assert_eq!(brain_luck(",>,< [ > [ >+ >+ << -] >> [- << + >>] <<< -] >>.", vec![8, 3]),  vec![24]);
}
