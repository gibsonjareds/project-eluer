#![feature(drain_filter)]


pub fn prime_factors(num: i64) -> Vec<i64> {
    let mut factors = vec![];
    let mut num2 = num;

        while num2 % 2 == 0 {
            num2 = num2 / 2;
            factors.push(2)
        }
        for i in (3..((num2 as f64).sqrt() as i64) + 1).step_by(2){
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
pub fn e_sieve (num: i64)-> Vec<i64> {
   let bound:i64 = (num as f64).sqrt() as i64 + 1;
   let mut factors:Vec<i64> = vec![];

    if num <= 1{
        return factors;
    }

    factors = (3..bound).step_by(2).collect::<Vec<i64>>();
    factors.insert(2, 0);
    let mut i = 1;
    while i < factors.len(){
        factors.drain_filter(|x| *x != factors[i] && *x % factors[i] ==0);
    }
    factors
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
}
