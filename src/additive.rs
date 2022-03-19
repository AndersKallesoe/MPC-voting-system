pub fn create_shares(secret: i64, prime: i64, number_of_servers: i64)-> Vec<i64>{
    let mut sum = 0;
    let mut shares: Vec<i64> = vec![];
        for _ in 0..number_of_servers-1{
            let share = random_share(prime);
            shares.push(share);
            sum += share;
        }
    let last_share = (prime + secret - (sum % prime)) % prime;
    shares.push(last_share);
    shares
}

/* generate a random share between 0 and prime */
fn random_share(prime: i64) -> i64{
    rand::random::<i64>() % prime
}