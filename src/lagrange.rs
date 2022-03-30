use num::rational::Ratio;

pub fn lagrange_interpolation(x_vec: &Vec<Ratio<i64>>, y_vec: &Vec<Ratio<i64>>, alpha: Ratio<i64>) -> Ratio<i64>{
    let n = x_vec.len();
    let zero: i64 = 0;
    let one: i64 = 1;
    let mut yp = Ratio::new(zero, one);
    for i in 0..n{
        let mut p = Ratio::new(one, one);
        for j in 0..n{
            if i!=j{p = p * (alpha - x_vec[j])/(x_vec[i] - x_vec[j]);}
        }
        yp = yp + p * y_vec[i];
    }
    return yp
}
// (x - a)(x - b)(x - c)
pub fn lagrange_coefficients(x_vec: &Vec<Ratio<i64>>, y_vec: &Vec<Ratio<i64>>) -> Vec<Ratio<i64>>{
    let num_of_points = x_vec.len();
    let mut sub_func_coefficients: Vec<Vec<Ratio<i64>>> = vec![];
    for j in 0..num_of_points{
        let mut coeffs: Vec<Ratio<i64>> = vec![];
        for i in 0..num_of_points{
            let coefficient_degree = (i as i64) + 1;
            coeffs.push(calc_coefficients(x_vec, j, coefficient_degree));
        }
        sub_func_coefficients.push(coeffs);
    }
    let num_of_coefficients = sub_func_coefficients.len();
    let mut weighted_sub_func_coefficients = vec![];
    let mut interpolated_coefficients = vec![Ratio::new(0, 1); num_of_coefficients];
    for (y, coeffs) in y_vec.iter().zip(sub_func_coefficients.iter()){
        let mut coeffs_ = vec![];
        for c in coeffs{
            coeffs_.push(c * y);
        }
        weighted_sub_func_coefficients.push(coeffs_);
    }
    for coeff in weighted_sub_func_coefficients{
        for i in 0..num_of_coefficients{
            interpolated_coefficients[i] += coeff[i];
        }
    }
    interpolated_coefficients.reverse();
    return interpolated_coefficients
}

fn calc_coefficients(x_vec: &Vec<Ratio<i64>>, j: usize, c: i64) -> Ratio<i64>{
    let mut negative_x = vec![];
    let mut divisor = Ratio::new(1,1);
    for (i, x) in x_vec.iter().enumerate(){
        if i == j{continue}
        divisor *= x_vec[j] - x;
        negative_x.push(-x)
    }
    if c == 1{ return Ratio::new(1, 1)/divisor }
    let mut sum_of_products = Ratio::new(0, 1);
    for combination in combinations(negative_x, c - 1){
        let mut product = Ratio::new(1, 1);
        for n in combination{
            product *= n;
        }
        sum_of_products += product;
    }
    return sum_of_products / divisor
}

fn combinations(x: Vec<Ratio<i64>>, r: i64) -> Vec<Vec<Ratio<i64>>>{
    let mut combination_vec = vec![];
    let mut combination_buffer = vec![Ratio::new(0, 1); r as usize]; //temporary buffer
    combinations_inner(&x, &mut combination_vec, &mut combination_buffer, 0, x.len() - 1, 0, r); //recursive loop over X
    combination_vec
}

fn combinations_inner(x_vec: &Vec<Ratio<i64>>, combination_vec: &mut Vec<Vec<Ratio<i64>>>, combination_buffer: &mut Vec<Ratio<i64>>, start: usize, end: usize, index: usize, r:i64){
    if index == r as usize{
        combination_vec.push(combination_buffer.to_vec()); //push copy of buffer to collection of possible combinations
        return
    }
    let mut i = start;
    let mut end_not_reached = true;
    let mut current_combination_not_finished = true;
    while end_not_reached && current_combination_not_finished{
        combination_buffer[index] = x_vec[i];
        combinations_inner(x_vec, combination_vec, combination_buffer, i + 1, end, index + 1, r);
        i += 1;
        end_not_reached = i <= end;
        current_combination_not_finished = (end + 1) - i + 1 >= r as usize - index + 1; // + 1 added to avoid usize overflow
    }
}