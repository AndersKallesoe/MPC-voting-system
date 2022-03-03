use num::rational::BigRational;
use num::rational::Ratio;
use num::bigint::BigInt;
use num_bigint::{ToBigInt, };

pub fn lagrange_interpolation(X: &Vec<Ratio<i64>>, C: &Vec<Ratio<i64>>, alpha: Ratio<i64>) -> Ratio<i64>{
    let n = X.len();
    let zero: i64 = 0;
    let one: i64 = 1;
    let mut yp = Ratio::new(zero, one);
    for i in 0..n{
        let mut p = Ratio::new(one, one);
        for j in 0..n{
            if i!=j{p = p * (alpha - X[j])/(X[i] - X[j]);}
        }
        yp = yp + p * C[i];
    }
    return yp
}