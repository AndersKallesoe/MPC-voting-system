

fn imodulo(a:i64, b:i64) -> i64{
    ((a % b) + b) % b
}

fn mod_inverse(a: i64, m: i64) -> i64{
    for i in 1..m{
        if (imodulo(imodulo(a, m) * imodulo(i, m), m)) == 1{
            return i as i64
        }
    }
    return -1
}

pub fn lagrange_interpolation(x_vec: Vec<i64>, y_vec: Vec<i64>, alpha: i64, prime: i64) -> i64{
    let n = x_vec.len();
    let mut yp = 0;
    for i in 0..n{
        let mut p = 1;
        for j in 0..n{
            if i!=j{
                let denom = imodulo(x_vec[i] - x_vec[j], prime);
                let num = imodulo(alpha - x_vec[j], prime);
                let denom_inverse = mod_inverse(denom, prime);
                p = imodulo(p * num * denom_inverse, prime);
            }
        }
        yp =imodulo( yp + p * y_vec[i],prime);
    }
    return imodulo(yp, prime)
}

pub fn lagrange_coefficients(x_vec: &Vec<i64>, y_vec: &Vec<i64>, prime: i64) -> Vec<i64>{
    let num_of_points = x_vec.len();
    let mut sub_func_coefficients: Vec<Vec<i64>> = vec![];
    for j in 0..num_of_points{
        let mut coeffs: Vec<i64> = vec![];
        for i in 0..num_of_points{
            let coefficient_degree = (i as i64) + 1;
            coeffs.push(calc_coefficients(x_vec, j, coefficient_degree, prime));
        }
        sub_func_coefficients.push(coeffs);
    }
    let num_of_coefficients = sub_func_coefficients.len();
    let mut weighted_sub_func_coefficients = vec![];
    let mut interpolated_coefficients = vec![0; num_of_coefficients];
    for (y, coeffs) in y_vec.iter().zip(sub_func_coefficients.iter()){
        let mut coeffs_ = vec![];
        for c in coeffs{
            coeffs_.push(imodulo(c * y,prime));
        }
        weighted_sub_func_coefficients.push(coeffs_);
    }
    for coeff in weighted_sub_func_coefficients{
        for i in 0..num_of_coefficients{
            interpolated_coefficients[i] = imodulo(interpolated_coefficients[i] + coeff[i],prime);
        }
    }
    interpolated_coefficients.reverse();
    return interpolated_coefficients
}

fn calc_coefficients(x_vec: &Vec<i64>, j: usize, c: i64, prime: i64) -> i64{
    let mut negative_x = vec![];
    let mut divisor = 1;
    for (i, x) in x_vec.iter().enumerate(){
        if i == j{continue}
        divisor *= imodulo (x_vec[j] - x,prime);
        negative_x.push(imodulo(-x,prime));
    }
    if c == 1{ return mod_inverse(divisor, prime) }
    let mut sum_of_products = 0;
    for combination in combinations(&negative_x, c - 1, prime){
        let mut product = 1;
        for n in combination{
            product *= n;
        }
        sum_of_products += product;
    }
    return sum_of_products * mod_inverse(divisor, prime)
}

fn combinations(x: &Vec<i64>, r: i64, prime: i64) -> Vec<Vec<i64>>{
    let mut combination_vec = vec![];
    let mut combination_buffer = vec![0; r as usize]; //temporary buffer
    combinations_inner(&x, &mut combination_vec, &mut combination_buffer, 0, x.len() - 1, 0, r, prime); //recursive loop over X
    combination_vec
}

fn combinations_inner(x_vec: &Vec<i64>, combination_vec: &mut Vec<Vec<i64>>, combination_buffer: &mut Vec<i64>, start: usize, end: usize, index: usize, r:i64, prime: i64){
    if index == r as usize{
        combination_vec.push(combination_buffer.to_vec()); //push copy of buffer to collection of possible combinations
        return
    }
    let mut i = start;
    let mut end_not_reached = true;
    let mut current_combination_not_finished = true;
    while end_not_reached && current_combination_not_finished{
        combination_buffer[index] = x_vec[i];
        combinations_inner(x_vec, combination_vec, combination_buffer, i + 1, end, index + 1, r, prime);
        i += 1;
        end_not_reached = i <= end;
        current_combination_not_finished = (end + 1) - i + 1 >= r as usize - index + 1; // + 1 added to avoid usize overflow
    }
}