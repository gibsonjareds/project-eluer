extern crate utils;

pub fn run(){
    println!("Problem 3: {}", solution(600851475143));
}

fn solution(num:u64) -> u64{
    let factors = utils::prime_factors(num);

    factors.iter().fold(u64::MIN, |a, &b| a.max(b))
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_base(){
        assert_eq!(solution(13195), 29);
    }

}
