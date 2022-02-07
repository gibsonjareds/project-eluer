extern crate utils;

pub fn run(){
    println!("Problem 7: {}", solution(10001));
}

fn solution(max:i64) -> i64{
    let mut primes_found = 1;
    let mut prime = 1;
    while primes_found < max {
        prime += 2;
        if utils::is_prime(prime) {
            primes_found += 1
        }
    }   
    prime
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_base(){
        assert_eq!(solution(6), 13);
    }

}
