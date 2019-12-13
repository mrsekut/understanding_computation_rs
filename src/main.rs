/// アンダースタンディングコンピューテーション 2章
/// スモールステップ意味論の実装
#[derive(Debug)]
enum Expr {
    Number(i32),
    Add(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn num(value: i32) -> Self {
        Expr::Number(value)
    }

    fn add(lhs: Expr, rhs: Expr) -> Self {
        Expr::Add(Box::new(lhs), Box::new(rhs))
    }

    fn mul(lhs: Expr, rhs: Expr) -> Self {
        Expr::Multiply(Box::new(lhs), Box::new(rhs))
    }

    /// 文字列に変換する
    fn to_s(&mut self) -> String {
        match self {
            Expr::Number(i) => format!("{}", i),
            Expr::Add(l, r) => format!("{} + {}", l.to_s(), r.to_s()),
            Expr::Multiply(l, r) => format!("{} * {}", l.to_s(), r.to_s()),
        }
    }

    /// 簡約可能かどうか
    fn is_reduction(&mut self) -> bool {
        match self {
            Expr::Number(_) => false,
            _ => true,
        }
    }

    fn reduce(&mut self) -> Self {
        match self {
            Expr::Add(l, r) => {
                unimplemented!
                // if l.is_reduction() {
                //     Self::add(l.reduce(), r.0)
                // } else if r.is_reduction() {
                //     Self::add(l, r.reduce())
                // } else {
                //     Self::num(l.0 + r.0)
                // }
            }
            Expr::Multiply(l, r) => unimplemented!,
            x => x,
        }
    }
}

fn main() {
    let n1 = Expr::num(1);
    let n2 = Expr::num(2);
    let n3 = Expr::num(2);
    let n4 = Expr::num(2);

    let m1 = Expr::mul(n1, n2);
    let m2 = Expr::mul(n3, n4);

    let mut a = Expr::add(m1, m2);
    println!("{}", a.is_reduction());
}
