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

pub fn lagrange_coefficients(X: &Vec<Ratio<i64>>, Y: &Vec<Ratio<i64>>) -> Vec<Ratio<i64>>{
    let n = X.len();
    let mut coefficients: Vec<Vec<Ratio<i64>>> = vec![];
    for j in 0..n{
        let mut coeffs: Vec<Ratio<i64>> = vec![];
        for i in 0..n{
            coeffs.push(calc_coefficients(X, j, (i as i64) + 1));
        }
        coefficients.push(coeffs);
    }
    println!("{:?}", coefficients);
    let mut weighted_coefficients = vec![];
    let mut actual_coefficients = vec![Ratio::new(0, 1); coefficients.len()];
    for (y, coeffs) in Y.iter().zip(coefficients.iter()){
        let mut coeffs_ = vec![];
        for c in coeffs{
            coeffs_.push(c * y);
        }
        weighted_coefficients.push(coeffs_);
    }
    for coeff in weighted_coefficients{
        for i in 0..coefficients.len(){
            actual_coefficients[i] += coeff[i];
        }
    }
    println!();
    return actual_coefficients
}

fn calc_coefficients(X: &Vec<Ratio<i64>>, j: usize, c: i64) -> Ratio<i64>{
    let mut minus_x = vec![];
    let mut divisor = Ratio::new(1,1);
    for (i, x) in X.iter().enumerate(){
        if i == j{continue}
        divisor *= X[j] - x;
        minus_x.push(-x)
    }
    if c == 1{ return Ratio::new(1, 1)/divisor }
    let mut sum_of_products = Ratio::new(0, 1);
    for comb in combinations(minus_x, c - 1){
        let mut product = Ratio::new(1, 1);
        for n in comb{
            product *= n;
        }
        sum_of_products += product;
    }
    return sum_of_products / divisor
}

fn combinations(X: Vec<Ratio<i64>>, r: i64) -> Vec<Vec<Ratio<i64>>>{
    let mut comb_vec = vec![];
    let mut tup = vec![Ratio::new(0, 1); r as usize];
    combinations_inner(&X, &mut comb_vec, &mut tup, 0, X.len() - 1, 0, r);
    comb_vec
}

fn combinations_inner(X: &Vec<Ratio<i64>>, comb_vec: &mut Vec<Vec<Ratio<i64>>>, tup: &mut Vec<Ratio<i64>>, start: usize, end: usize, index: usize, r:i64){
    if index == r as usize{
        comb_vec.push(tup.to_vec());
        return
    }
    let mut i = start;
    while (i <= end && end - i + 1 >= r as usize - index){
        tup[index] = X[i];
        combinations_inner(X, comb_vec, tup, i + 1, end, index + 1, r);
        i += 1;
    }
}