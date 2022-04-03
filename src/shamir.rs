use num::rational::Ratio;

pub fn create_shares(coefficients: &Vec<i64>, number_of_servers: i64) -> Vec<i64>{
    let mut ys = vec![];
    for x in 1..number_of_servers + 1{
        let y = evaluate(&coefficients[..], x);
        ys.push(y);
    }
    ys
}
pub fn create_coefficients(secret: i64, t: i64, prime: u64) -> Vec<i64>{
    let mut coeffs = vec![secret];
    for _ in 0..t{
        coeffs.push((rand::random::<u64>() % prime) as i64); 
    }
    coeffs
}

pub fn recover_secret(shares: &[i64]) -> i64{
    let share_vec = shares.iter().map(|s|Ratio::new(*s, 1)).collect::<Vec<Ratio<i64>>>();
    let x_vec = (1..shares.len() as i64 + 1).map(|x|Ratio::new(x, 1)).collect::<Vec<Ratio<i64>>>();
    let result_as_ratio = crate::lagrange::lagrange_interpolation(&x_vec, &share_vec, Ratio::new(0, 1));
    match * result_as_ratio.denom(){
        1 => {*result_as_ratio.numer()}
        _ => {-2}
    } //TODO: Propagate cases where denominator is not 1 further in the system
}

fn recover_coefficients(shares: &[i64]) -> Vec<i64>{
    let share_vec = shares.iter().map(|s|Ratio::new(*s, 1)).collect::<Vec<Ratio<i64>>>();
    let x_vec = (1..shares.len() as i64 + 1).map(|x|Ratio::new(x, 1)).collect::<Vec<Ratio<i64>>>();
    let coeffs_as_ratios = crate::lagrange::lagrange_coefficients(&x_vec, &share_vec);
    let mut result = vec![];
    for coeff in coeffs_as_ratios.iter(){
        match *coeff.denom(){
            1=> {result.push(*coeff.numer());}
            _=> return vec![-2]
        }
    }; //TODO: Propagate cases where denominator is not 1 further in the system
    result
}
//[x*2 for x in numbers]
fn verify(coefficients: &[i64], x: i64, y: i64) -> bool{
    let y_1 = evaluate(coefficients, x);
    y_1 == y
}

fn evaluate(coefficients: &[i64], x: i64) -> i64{
    let mut sum = 0;
    for (i, c) in coefficients.iter().enumerate(){
        sum += c * x.pow(i as u32);
    }
    return sum
}

pub fn fault_detection(shares: &[i64]) -> i64{
    let servers = shares.len();
    let degree = detection_degree(servers as u8) as usize;
    let coefficients = recover_coefficients(&shares[..degree+1]);
    if coefficients != vec![-2]{
        for i in degree+1..servers{
            if !verify(&coefficients, (i + 1) as i64, shares[i]){
                return -1
            }
        }
    }
    coefficients[0]
}

pub fn detection_degree(servers: u8)->i64{
    let corruptions = (servers - 1) / 2;
    if (servers - 1) % 2 == 0{
        return corruptions as i64
    }
    return (corruptions + 1) as i64
}

pub fn correction_degree(servers: u8)->u8{
    let corruptions = (servers - 1) / 3;
    if (servers - 1) % 3 == 0{
        return corruptions
    }
    return corruptions + 1
}
