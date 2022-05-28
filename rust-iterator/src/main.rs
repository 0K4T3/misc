struct FizzBuzz {
    current: u32,
    to: u32
}

impl FizzBuzz {
    fn new(from: u32, to: u32) -> Self {
	FizzBuzz {
	    current: from,
	    to,
	}
    }

    fn judge(&self, num: u32) -> String {
	let mut buffer = String::new();

	if num % 3 == 0 { buffer.push_str(&String::from("Fizz")); }
	if num % 5 == 0 { buffer.push_str(&String::from("Buzz")); }
	if buffer.len() < 1 { buffer.push_str(&num.to_string()) }

	buffer
    }
}

impl Iterator for FizzBuzz {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
	if self.current < self.to {
	    let result = self.judge(self.current);
	    self.current += 1;
	    Some(result)
	} else {
	    None
	}
    }
}

fn main() {
    const FIZZ_BUZZ_MAX: u32 = 100;

    let fizz_buzz = FizzBuzz::new(0, FIZZ_BUZZ_MAX);
    for s in fizz_buzz {
	println!("{}", s);
    }
}
