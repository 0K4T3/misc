type SimpleError = Box<dyn std::error::Error>;

trait StackMachine {
    // Run compiled codes
    fn run(&mut self) -> Result<(), SimpleError>;

    // Compile tokens to operators
    fn compile(&mut self, source: &str) -> Result<(), SimpleError>;
}

enum Operator {
    Push(i64),
    Pop,
    Debug,
    Nop,
}

struct SimpleStackMachine {
    // Instruction pointer is pointing address to executing location on a memory
    ip: usize,
    // Stack pointer is pointing address to top of stack
    sp: i64,
    // General register to accumulate instruction result
    accumulator: i64,
    // 
    stack: Vec<i64>,
    //
    code: Vec<Operator>,
}

impl SimpleStackMachine {
    fn new() -> Self {
        Self {
            ip: 0,
            sp: -1,
            accumulator: 0,
            stack: Vec::new(),
            code: Vec::new(),
        }
    }
}

impl StackMachine for SimpleStackMachine {
    fn run(&mut self) -> Result<(), SimpleError> {
        loop {
            let op = match self.code.get(self.ip) {
                Some(op) => op,
                None => break,
            };

            match op {
                Operator::Push(value) => {
                    self.sp += 1;
                    self.stack.push(*value);
                },
                Operator::Pop => {
                    self.accumulator = self.stack.pop().unwrap();
                    self.sp -= 1;
                },
                Operator::Debug => {
                    println!("===== DEBUG =====");
                    println!("IP: {}", self.ip);
                    println!("SP: {}", self.sp);
                    println!("STACK: {:?}", self.stack);
                },
                Operator::Nop => {},
            }

            self.ip += 1;
        }

        Ok(())
    }

    fn compile(&mut self, source: &str) -> Result<(), SimpleError> {
        let tokens: Vec<&str> = source.split(';').collect();

        self.code = tokens.iter().map(|token| {
            if token.starts_with("PUSH ") {
                Operator::Push(100)
            } else if *token == "POP" {
                Operator::Pop
            } else if *token == "DEBUG" {
                Operator::Debug
            } else if *token == "NOP" {
                Operator::Nop
            } else {
                Operator::Nop
            }
        }).collect();

        Ok(())
    }
}

fn main() -> Result<(), SimpleError> {
    let mut ssm = SimpleStackMachine::new();
    let source = "DEBUG;PUSH 10;DEBUG;POP;DEBUG";

    ssm.compile(source)?;
    ssm.run()?;

    Ok(())
}
