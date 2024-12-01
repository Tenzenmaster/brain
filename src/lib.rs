#[derive(Clone, Copy, Debug, Default)]
enum Opcode {
    #[default]
    Noop,
    IncPtr,
    DecPtr,
    IncCell,
    DecCell,
    Output,
    Input,
    LoopStart(usize),
    LoopEnd(usize),
}

pub struct Program {
    code: Vec<Opcode>,
    ip: usize,
    ptr: usize,
    cells: [u8; Self::CELL_COUNT],
    input: Vec<u8>,
}

impl Program {
    const CELL_COUNT: usize = 32000;

    pub fn new(source: &str, input: &str) -> Self {
        let mut ret = Self {
            code: Vec::new(),
            ip: 0,
            ptr: Self::CELL_COUNT / 2,
            cells: [0; Self::CELL_COUNT],
            input: input.as_bytes().to_owned(),
        };

        assert!(source.is_ascii());
        assert!(input.is_ascii());

        let mut loop_stack = Vec::new();

        for b in source.bytes() {
            match b {
                b'>' => ret.code.push(Opcode::IncPtr),
                b'<' => ret.code.push(Opcode::DecPtr),
                b'+' => ret.code.push(Opcode::IncCell),
                b'-' => ret.code.push(Opcode::DecCell),
                b'.' => ret.code.push(Opcode::Output),
                b',' => ret.code.push(Opcode::Input),
                b'[' => {
                    loop_stack.push(ret.code.len());
                    ret.code.push(Opcode::LoopStart(0));
                }
                b']' => {
                    let len = ret.code.len();
                    let start_index = loop_stack.pop().unwrap();
                    let loop_start = ret.code.get_mut(start_index).unwrap();
                    let Opcode::LoopStart(i) = loop_start else {
                        panic!("{loop_start:?} is not a LoopStart");
                    };
                    *i = len;
                    ret.code.push(Opcode::LoopEnd(start_index));
                }
                _ => (),
            }
        }

        ret
    }

    pub fn run(&mut self) {
        let mut input = self.input.iter();
        while let Some(code) = self.code.get(self.ip) {
            match code {
                Opcode::Noop => (),
                Opcode::IncPtr => self.ptr = self.ptr.wrapping_add(1),
                Opcode::DecPtr => self.ptr = self.ptr.wrapping_sub(1),
                Opcode::IncCell => self.cells[self.ptr] = self.cells[self.ptr].wrapping_add(1),
                Opcode::DecCell => self.cells[self.ptr] = self.cells[self.ptr].wrapping_sub(1),
                Opcode::Output => print!("{}", self.cells[self.ptr] as char),
                Opcode::Input => {
                    self.cells[self.ptr] = *input.next().expect("Not enough bytes in input")
                }
                Opcode::LoopStart(i) => {
                    if self.cells[self.ptr] == 0 {
                        self.ip = *i;
                    }
                }
                Opcode::LoopEnd(i) => {
                    if self.cells[self.ptr] != 0 {
                        self.ip = *i;
                    }
                }
            }
            self.ip += 1;
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        Program::new("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.", "")
            .run();

        Program::new("+++++++++++[>++++++>+++++++++>++++++++>++++>+++>+<<<<<<-]>++++++.>++.+++++++..+++.>>.>-.<<-.<.+++.------.--------.>>>+.>-.", "")
            .run();

        Program::new("--<-<<+[+[<+>--->->->-<<<]>]<<--.<++++++.<<-..<<.<+.>>.>>.<<<.+++.>>.>>-.<<<+.", "")
            .run();

        Program::new("+[-->-[>>+>-----<<]<--<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+.", "")
            .run();
    }

    #[test]
    fn evil_hello_world() {
        Program::new(">++++++++[-<+++++++++>]<.>>+>-[+]++>++>+++[>[->+++<<+++>]<<]>-----.>->+++..+++.>-.<<+[>[+>+]>>]<--------------.>>.+++.------.--------.>+.>+.", "")
            .run();
    }

    #[test]
    fn cat_null_termination() {
        Program::new(",[.,]", "hello hehe\0").run();
    }

    #[test]
    fn cell_size() {
        Program::new(
            r#"
Calculate the value 256 and test if it's zero
If the interpreter errors on overflow this is where it'll happen
++++++++[>++++++++<-]>[<++++>-]
+<[>-<
    Not zero so multiply by 256 again to get 65536
    [>++++<-]>[<++++++++>-]<[>++++++++<-]
    +>[>
        # Print "32"
        ++++++++++[>+++++<-]>+.-.[-]<
    <[-]<->] <[>>
        # Print "16"
        +++++++[>+++++++<-]>.+++++.[-]<
<<-]] >[>
    # Print "8"
    ++++++++[>+++++++<-]>.[-]<
<-]<
# Print " bit cells\n"
+++++++++++[>+++>+++++++++>+++++++++>+<<<<-]>-.>-.+++++++.+++++++++++.<.
>>.++.+++++++..<-.>>-.
Clean up used cells
[[-]<]
            "#,
            "",
        )
        .run();
    }
}
