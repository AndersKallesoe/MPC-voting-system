use num::rational::Ratio;

pub fn create_shares(secret: i64, prime: i64, number_of_servers: i64) -> Vec<i64>{
    let coefficients = create_coefficients(secret, number_of_servers-1, prime as u64);
    let mut ys = vec![];
    for x in 1..number_of_servers{
        let mut y = 0;
        for (i, c) in coefficients.iter().enumerate(){
            y += c * x.pow(i as u32);
        }
        ys.push(y);
    }
    ys
}
fn create_coefficients(secret: i64, t: i64, prime: u64) -> Vec<i64>{
    let mut coeffs = vec![secret];
    for _ in 0..t{
        coeffs.push((rand::random::<u64>() % prime) as i64); 
    }
    coeffs
}

pub fn recover_secret(shares: &Vec<i64>) -> i64{
    let share_vec = shares.iter().map(|s|Ratio::new(*s, 1)).collect::<Vec<Ratio<i64>>>();
    let x_vec = (1..shares.len() as i64 + 1).map(|x|Ratio::new(x, 1)).collect::<Vec<Ratio<i64>>>();
    let result_as_ratio = crate::lagrange::lagrange_interpolation(&x_vec, &share_vec, Ratio::new(0, 1));
    *result_as_ratio.numer()
}

pub fn recover_coefficients(shares: &Vec<i64>) -> Vec<i64>{
    let share_vec = shares.iter().map(|s|Ratio::new(*s, 1)).collect::<Vec<Ratio<i64>>>();
    let x_vec = (1..shares.len() as i64 + 1).map(|x|Ratio::new(x, 1)).collect::<Vec<Ratio<i64>>>();
    let coeffs_as_ratios = crate::lagrange::lagrange_coefficients(&x_vec, &share_vec);
    coeffs_as_ratios.iter().map(|c|*c.numer()).collect::<Vec<i64>>()
}
