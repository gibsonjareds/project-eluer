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


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_can_get_primes(){
        assert_eq!(prime_factors(13195), vec![5, 7, 13, 29])
    }
    #[test]
    fn test_nine(){
        assert_eq!(prime_factors(9), vec![3,3])
    }

}
