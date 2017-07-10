fn main() {
    println!(
        "{:?}",
        brain_luck(
            ",>+>>>>++++++++++++++++++++++++++++++++++++++++++++>++++++++++++++++++++++++++++++++<<<<<<[>[>>>>>>+>+<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<[>++++++++++[-<-[>>+>+<<<-]>>>[<<<+>>>-]+<[>[-]<[-]]>[<<[>>>+<<<-]>>[-]]<<]>>>[>>+>+<<<-]>>>[<<<+>>>-]+<[>[-]<[-]]>[<<+>>[-]]<<<<<<<]>>>>>[++++++++++++++++++++++++++++++++++++++++++++++++.[-]]++++++++++<[->-<]>++++++++++++++++++++++++++++++++++++++++++++++++.[-]<<<<<<<<<<<<[>>>+>+<<<<-]>>>>[<<<<+>>>>-]<-[>>.>.<<<[-]]<<[>>+>+<<<-]>>>[<<<+>>>-]<<[<+>-]>[<+>-]<<<-]",
            vec![2]
        )
    );
}

// advance_tests panics with nowhere to jump ?????
// multiply_tests overflow


fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let mut i = 0;
    let mut stack_ptr = 0;
    let mut stack_ptri : i32 = 0;

    let mut jmp = false;

    let mut jumps = vec![];
    let mut stack: Vec<u8> = vec![0];
    let mut nstack: Vec<u8> = vec![0];
    let mut output: Vec<u8> = vec![];
    let mut params = input.into_iter();

    loop {
        if let Some(op) = code.chars().nth(i) {

            let t_stack = select_stack(&mut nstack, &mut stack, stack_ptri);

            // jump until
            if jmp {
                // end jump
                if op == ']' {
                    jmp = false;
                    println!("");
                }
            } else {
                match op {
                    '[' => {
                        // jump wihtout executing between
                        if t_stack[index(stack_ptri)] {
                            jmp = true;
                        } else {
                            // set jump target
                            jumps.push(i);
                        }
                    }
                    ']' => {
                        jmp = false;

                        if 0 == 0 {
                            // no need to jump back, remove last jump addr and continue
                            jumps.pop();
                        } else {
                            // return to last jump
                            if jumps.len() > 0 {
                                i = jumps[jumps.len() - 1] - 1;
                                jumps.pop();
                            } else {
                                panic!("Nowhere to jump");
                            }
                        }
                    }
                    '.' => {
                        output.push(0);
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
        } else {
            break;
        }
    }

    output
}

fn index(i : i32) -> usize {
    if i > 0 {
        i as usize
    } else {
        (-i) as usize
    }
}

fn select_stack<'a>(neg: &'a mut Vec<u8>,pos: &'a mut Vec<u8>, i: i32) -> &'a mut Vec<u8> {
    if i > 0 {
        pos
    } else {
        neg
    }
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
fn a() {
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
            vec![8, 3]
        ),
        vec![24]
    );
}
#[test]
fn hello_world() {
    assert_eq!(
        brain_luck( "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.",
            vec![]
        ),
        "Hello World!\n".as_bytes()
    );
}

#[test]
fn hello_world_s() {
    assert_eq!(
        brain_luck(
"++++++++++
[>+++++++>++++++++++>+++>+<<<<-] The initial loop to set up useful values in the array
>++.                             Print 'H'
>+.                              Print 'e'
+++++++.                         Print 'l'
.                                Print 'l'
+++.                             Print 'o'
>++.                             Print ' '
<<+++++++++++++++.               Print 'W'
>.                               Print 'o'
+++.                             Print 'r'
------.                          Print 'l'
--------.                        Print 'd'
>+.                              Print '!'
>.                               Print newline",
            vec![]
        ),
        "Hello World!\n".as_bytes()
    );
}