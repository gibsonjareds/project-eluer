#![feature(drain_filter)]


pub fn prime_factors(num: u64) -> Vec<u64> {
    let mut factors = vec![];
    let mut num2 = num;

        while num2 % 2 == 0 {
            num2 = num2 / 2;
            factors.push(2)
        }
        for i in (3..((num2 as f64).sqrt() as u64) + 1).step_by(2){
            while num2 % i == 0{
                num2 = num2 / i;
                factors.push(i);
            }
        }

        if num2 > 2  {
            factors.push(num2);

        }

    factors

}
pub fn e_sieve (num: u64)-> Option<Vec<u64>> {
    let mut is_prime = vec![true;num as usize];
    let mut primes = vec![];
    // 1 and lower have no real prime factors
    if num <= 1 {
        return None;
    }
    if num == 2 {
        return Some(vec![])
    }
    for i in 2..num as usize {
        if !is_prime[i] {
            continue;
        }
        for e in ((i * i)..num as usize).step_by(i){
            is_prime[e] = false;
        }
        primes.push(i as u64);
    }
    Some(primes)
}
pub fn is_prime( n:i64) -> bool{
    if n == 1{
        return false;
    }
    if n < 4 {
        return true;
    }
    if n % 2 == 0{
        return false;
    }
    if n < 9{
        return true;
    }
    if n % 3 == 0 {
        return false;
    }
    let r = (n as f64).sqrt() as i64;
    let mut f = 5;
    let mut prime = true;
    while f <= r {
        if n % f == 0 {
            prime = false;
            break;
        } 
        if n % (f+2) == 0 {
            prime = false;
            break;
        }
        f += 6;
    }
    prime
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_can_get_prime_factors(){
        assert_eq!(prime_factors(13195), vec![5, 7, 13, 29])
    }
    #[test]
    fn test_nine_factors(){
        assert_eq!(prime_factors(9), vec![3,3])
    }
    #[test]
    fn test_13_is_prime(){
        assert_eq!(is_prime(13), true)
    }
    #[test]
    fn test_15_is_not_prime(){
        assert_eq!(is_prime(15), false)
    }
    #[test]
    fn test_primes_under_nine_sieve(){
        assert_eq!(e_sieve(9).unwrap(), vec![2,3,5,7]);
    }
}
