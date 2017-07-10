fn main() {
    println!(
        "{:?}",
        brain_luck(
            ",>,< [ > [ >+ >+ << -] >> [- << + >>] <<< -] >>.",
            vec![8, 9]
        )
    );
}

fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let mut i = 0;
    let mut stack_ptr = 0;

    let mut jmp = false;

    let mut jumps = vec![];
    let mut stack: Vec<u8> = vec![0];
    let mut output: Vec<u8> = vec![];
    let mut params = input.into_iter();

    loop {
        if let Some(op) = code.chars().nth(i) {
            // jump until
            if jmp {
                // end jump
                if op == ']' {
                    jmp = false;
                    println!("");
                }
            } else {
                if let Some(p_op) = code.chars().nth(i + 1) {
                    match p_op {
                        '\'' => {
                            i += 1;
                            continue;
                            },
                        '\"' => {
                            i += 1;
                            continue;
                            },
                        _ => {}
                    }
                }
                println!("{}:{}", i, op);
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
                            // no need to jump back, remove last jump addr and continue
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
                        stack_ptr -= 1;
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

#[test]
fn hello_world_w() {
    assert_eq!(
        brain_luck(
"[This program prints 'Hello World!' and a newline to the screen, its
  length is 106 active command characters'.' '['It is not the shortest'.']'

  This loop is an 'initial comment loop'',' a simple way of adding a comment
  to a BF program such that you don't have to worry about any command
  characters'.' Any '.',' ',',' '+'',' '-'',' '<' and '>' characters are simply
  ignored',' the '[' and ']' characters just have to be balanced'.' This
  loop and the commands it contains are ignored because the current cell
  defaults to a value of 0; the 0 value causes this loop to be skipped'.'
]
++++++++               Set Cell #0 to 8
[
    >++++               Add 4 to Cell #1; this will always set Cell #1 to 4
    [                   as the cell will be cleared by the loop
        >++             Add 2 to Cell #2
        >+++            Add 3 to Cell #3
        >+++            Add 3 to Cell #4
        >+              Add 1 to Cell #5
        <<<<-           Decrement the loop counter in Cell #1
    ]                   Loop till Cell #1 is zero; number of iterations is 4
    >+                  Add 1 to Cell #2
    >+                  Add 1 to Cell #3
    >-                  Subtract 1 from Cell #4
    >>+                 Add 1 to Cell #6
    [<]                 Move back to the first zero cell you find; this will
                        be Cell #1 which was cleared by the previous loop
    <-                  Decrement the loop Counter in Cell #0
]                       Loop till Cell #0 is zero; number of iterations is 8

The result of this is:
Cell No :   0   1   2   3   4   5   6
Contents:   0   0  72 104  88  32   8
Pointer :   ^

>>.                     Cell #2 has value 72 which is 'H'
>---.                   Subtract 3 from Cell #3 to get 101 which is 'e'
+++++++..+++.           Likewise for 'llo' from Cell #3
>>.                     Cell #5 is 32 for the space
<-.                     Subtract 1 from Cell #4 for 87 to give a 'W'
<.                      Cell #3 was set to 'o' from the end of 'Hello'
+++.------.--------.    Cell #3 for 'rl' and 'd'
>>+.                    Add 1 to Cell #5 gives us an exclamation point
>++.                    And finally a newline from Cell #6",
            vec![]
        ),
        "Hello World!\n".as_bytes()
    );
}
