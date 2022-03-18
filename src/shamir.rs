use num::rational::Ratio;
pub fn create_secret(s: i64, t: i64, prime: u64) -> Vec<i64>{
    let mut coeffs = vec![s];
    for _ in 0..t{
        coeffs.push((rand::random::<u64>() % prime) as i64); 
    }
    coeffs
}
pub fn create_shares(coeffs: &Vec<i64>, xs: &Vec<i64>) -> Vec<i64>{
    let mut ys = vec![];
    for x in xs.iter(){
        let mut y = 0;
        for (i, c) in coeffs.iter().enumerate(){
            y += c * x.pow(i as u32);
        }
        ys.push(y);
    }
    ys
}
pub fn recover_secret(ys: &Vec<i64>, xs: &Vec<i64>) -> i64{
    let y_vec = ys.iter().map(|y|Ratio::new(*y, 1)).collect::<Vec<Ratio<i64>>>();
    let x_vec = xs.iter().map(|x|Ratio::new(*x, 1)).collect::<Vec<Ratio<i64>>>();
    let res_rat = crate::lagrange::lagrange_interpolation(&x_vec, &y_vec, Ratio::new(0, 1));
    *res_rat.numer()
}

pub fn recover_coefficients(ys: &Vec<i64>, xs: &Vec<i64>) -> Vec<i64>{
    let y_vec = ys.iter().map(|y|Ratio::new(*y, 1)).collect::<Vec<Ratio<i64>>>();
    let x_vec = xs.iter().map(|x|Ratio::new(*x, 1)).collect::<Vec<Ratio<i64>>>();
    let rat_coeffs = crate::lagrange::lagrange_coefficients(&x_vec, &y_vec);
    rat_coeffs.iter().map(|c|*c.numer()).collect::<Vec<i64>>()
}
