const P: u64 = 59;  // ここを適宜書き換える



pub mod identities;
pub mod inverse;
pub mod modint;
pub mod polynomial;
pub mod solution_set;

use crate::modint::ModInt;
use crate::identities::Zero;
use crate::polynomial::Polynomial;
use crate::solution_set::SolutionSet;

use std::collections::HashSet;

fn main() {
    println!("mod {} での多項式の根を求めます。", P);

    if !is_prime(P) {
        println!("注：{}は素数ではありません。",P);
    }

    // 解く方程式の左辺の次数
    println!("根を求めたい多項式の次数を入力");
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    // usize型に変換
    let n: usize = n.trim().parse().ok().unwrap();

    let mut v: Vec<ModInt<P>> = vec![ModInt::<P>::zero();n+1];

    for i in 0..=n {
        println!("{}次の係数を入力",i);
        let mut a = String::new();
        std::io::stdin().read_line(&mut a).ok();
        // u64型に変換
        let a: u64 = a.trim().parse().ok().unwrap();
        v[i] = ModInt::<P>::new(a);
    }

    let f: Polynomial<ModInt<P>> = Polynomial::new(&v);

    let s: SolutionSet<ModInt<P>> = solve_equation(&f);

    println!("多項式 {} の根の集合は", f);
    println!("{}",s);
    println!("です。")
}

/// 素数判定
fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    if n == 0 || n == 1 {
        return false;
    }
    for i in 0..n {
        if n != 3 + 2 * i && n % (3 + 2 * i) == 0 {
            return false;
        }
        if (3 + 2 * i) * (3 + 2 * i) >= n {
            break;
        }
    }
    true
}

/// 方程式の解を全探索
fn solve_equation(f: &Polynomial<ModInt<P>>) -> SolutionSet<ModInt<P>> {
    let mut s: HashSet<ModInt<P>> = HashSet::new();
    for i in 0..P {
        if Polynomial::evaluate(&f, ModInt::<P>::new(i)) == ModInt::<P>::zero() {
            s.insert(ModInt::<P>::new(i));
        }
    }
    SolutionSet::new(s)
}

