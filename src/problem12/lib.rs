extern crate utils;



pub fn run(){
    println!("Problem 12: {}", solution(501));
}

fn solution(number_of_divisors:usize) -> i64{
    if number_of_divisors == 1{
        return 1;
    }

    let mut n = 1;
    let mut num:i64 = 1;
    let mut prod = 1;
    while prod <= number_of_divisors {
        num = n * (n+1) / 2;
        prod = 1;
        let mut factors = utils::prime_factors(num as u64);
        factors.sort();
        while factors.len() > 1{
            let index = factors.iter().rposition(|&x| x == factors[0]).unwrap();
            if index > 0 {
                let subfactors:Vec<u64> = factors.drain(0..(index+1)).collect();
                prod *= subfactors.len() + 1;
            }else{
                prod *= 2;
                factors.remove(0);
            }
        }
        prod*= factors.len() + 1;

        n += 1
    }

    num
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_base(){
        assert_eq!(solution(5), 28);
    }
    #[test]
    fn test_base_two(){
        assert_eq!(solution(2), 6);
    }
    #[test]
    fn test_base_three(){
        assert_eq!(solution(15), 120);
    }

}
