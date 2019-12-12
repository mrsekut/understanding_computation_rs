#[derive(Debug)]
struct Number {
    value: i32,
}

impl Number {
    fn new(value: i32) -> Self {
        Number { value }
    }

    fn to_s(&mut self) -> String {
        format!("{}", self.value)
    }
}

#[derive(Debug)]
struct Add {
    lhs: String,
    rhs: String,
}

impl Add {
    fn new(lhs: String, rhs: String) -> Self {
        Add { lhs, rhs }
    }

    fn to_s(&mut self) -> String {
        format!("{} + {}", self.lhs, self.rhs)
    }
}

#[derive(Debug)]
struct Multiply {
    lhs: String,
    rhs: String,
}

impl Multiply {
    fn new(lhs: String, rhs: String) -> Self {
        Multiply { lhs, rhs }
    }

    fn to_s(&mut self) -> String {
        format!("{} * {}", self.lhs, self.rhs)
    }
}

fn main() {
    let n1 = Number::new(1).to_s();
    let n2 = Number::new(2).to_s();
    let n3 = Number::new(1).to_s();
    let n4 = Number::new(2).to_s();

    let m1 = Multiply::new(n1, n2).to_s();
    let m2 = Multiply::new(n3, n4).to_s();

    let mut a = Add::new(m1, m2);
    println!("{}", a.to_s())
}
