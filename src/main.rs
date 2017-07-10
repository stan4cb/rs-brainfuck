fn main() {}

// advance_tests panics with nowhere to jump ?????
// multiply_tests overflow


fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let mut i = 0;
    let mut stack_ptr = 0;

    let mut jmp_depth = 0;

    let mut jumps = vec![];
    let mut stack: Vec<u8> = vec![0];
    let mut output: Vec<u8> = vec![];
    let mut params = input.into_iter();
    let chars = code.as_bytes();

    loop {
        if i >= chars.len() {
            break;
        }

        let op = chars[i] as char;

        // jump until
        if jmp_depth != 0 {
            // end jump
            match op {
                '[' => jmp_depth += 1,
                ']' => jmp_depth -= 1,
                _ => {}
            }
        } else {
            match op {
                '[' => {
                    // jump wihtout executing between
                    if stack[stack_ptr] == 0 {
                        jmp_depth += 1;
                    } else {
                        // set jump target
                        jumps.push(i);
                    }
                }
                ']' => {
                    if stack[stack_ptr] == 0 {
                        // no need to jump back, remove last jump addr and continue
                        jumps.pop();
                    } else {
                        // return to last jump
                        if jumps.len() > 0 {
                            let pos = jumps.pop().unwrap();
                            i = pos - 1;
                        } else {
                            panic!("Nowhere to jump");
                        }
                    }
                }
                '.' => {
                    output.push(stack[stack_ptr]);
                }
                ',' => {
                    stack[stack_ptr] = params.next().expect("No param left");
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
                    if stack_ptr > 0 {
                        stack_ptr -= 1;
                    } else {
                        panic!("Negative stack not implemented");
                    }
                }
                _ => {}
            }
        }
        i += 1;
    }

    output
}

#[test]
fn read_until_zero() {
    assert_eq!(brain_luck(",[.[-],]", vec![1, 1, 1, 0]), vec![1, 1, 1]);
}
#[test]
fn read_until_255() {
    assert_eq!(brain_luck(",+[-.,+]", vec![1, 1, 1, 255]), vec![1, 1, 1]);
}
#[test]
fn print_a() {
    assert_eq!(
        brain_luck("++++++ [ > ++++++++++ < - ] > +++++ .", vec![]),
        vec![65]
    );
}
#[test]
fn subtract() {
    assert_eq!(brain_luck(",>,[-<->]<.", vec![4, 3]), vec![1]);
}
#[test]
fn multiply() {
    assert_eq!(
        brain_luck(
            ",>,< [ > [ >+ >+ << -] >> [- << + >>] <<< -] >>.",
            vec![8, 3],
        ),
        vec![24]
    );
}
#[test]
fn hello_world() {
    assert_eq!(
        brain_luck(
            "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]
            >>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.",
            vec![],
        ),
        "Hello World!\n".as_bytes()
    );
}

#[test]
fn division() {
    assert_eq!(
        brain_luck(
            "
,>,>++++++[-<--------<-------->>]
<<[
>[->+>+<<]
>[-<<-
[>]>>>[<[>>>-<<<[-]]>>]<<]
>>>+
<<[-<<+>>]
<<<]
>[-]>>>>[-<<<<<+>>>>>]
<<<<++++++[-<++++++++>]<.
",
            vec![2, 1],
        ),
        vec![2]
    );
}
