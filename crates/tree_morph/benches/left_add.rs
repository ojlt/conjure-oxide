use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tree_morph::prelude::*;
use uniplate::derive::Uniplate;

#[derive(Debug, Clone, PartialEq, Eq, Uniplate)]
#[uniplate()]
enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Val(i32),
}

fn rule_eval_add(_: &mut Commands<Expr, Meta>, expr: &Expr, _: &Meta) -> Option<Expr> {
    match expr {
        Expr::Add(a, b) => match (a.as_ref(), b.as_ref()) {
            (Expr::Val(x), Expr::Val(y)) => Some(Expr::Val(x + y)),
            _ => None,
        },
        _ => None,
    }
}

fn rule_eval_mul(_: &mut Commands<Expr, Meta>, expr: &Expr, _: &Meta) -> Option<Expr> {
    match expr {
        Expr::Mul(a, b) => match (a.as_ref(), b.as_ref()) {
            (Expr::Val(x), Expr::Val(y)) => Some(Expr::Val(x * y)),
            _ => None,
        },
        _ => None,
    }
}

#[derive(Clone)]
enum MyRule {
    EvalAdd,
    EvalMul,
}

impl Rule<Expr, Meta> for MyRule {
    fn apply(&self, cmd: &mut Commands<Expr, Meta>, expr: &Expr, meta: &Meta) -> Option<Expr> {
        cmd.mut_meta(|m| m.num_applications += 1); // Only applied if successful
        match self {
            MyRule::EvalAdd => rule_eval_add(cmd, expr, meta),
            MyRule::EvalMul => rule_eval_mul(cmd, expr, meta),
        }
    }
}

#[derive(Clone)]
struct Meta {
    num_applications: u32,
}

fn val(n: i32) -> Box<Expr> {
    Box::new(Expr::Val(n))
}

fn add(lhs: Box<Expr>, rhs: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Add(lhs, rhs))
}

fn mul(lhs: Box<Expr>, rhs: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Mul(lhs, rhs))
}

fn nested_addition(n: usize) -> Box<Expr> {
    if n == 1 {
        val(1)
    } else {
        add(val(1), nested_addition(n - 1))
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let expr = *nested_addition(25);
    let rules = vec![vec![MyRule::EvalAdd, MyRule::EvalMul]];

    c.bench_function("left_add", |b| {
        b.iter(|| {
            let meta = Meta {
                num_applications: 0,
            };
            morph(rules.clone(), select_first, black_box(expr.clone()), meta)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
