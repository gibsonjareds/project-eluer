#![feature(int_log)]

extern crate utils;

pub fn run(){
    println!("Problem 5: {}", solution(20));
}

fn solution(max:u64) -> u64{
    let mut num:u64 = 1;
    let limit:u64 = (max as f64).sqrt() as u64;
    let mut check = true;
    let mut factors = Vec::new();
    for i in 2..(max+1) {
        let mut vec2 = utils::prime_factors(i);
        if factors.len() > 0{ 
        while vec2.len() > 0{
            for i in 0..factors.len() {
                let factor = factors[i];
                if vec2.len() > 0 {
                    if &factor == vec2.last().unwrap(){
                        vec2.pop();
                    }else if i+1 == factors.len(){
                        factors.push(vec2.pop().unwrap());
                    }
                }else{
                    break;
                }
            }
        }
        }else{
            factors.append(&mut vec2);
        }
    }
    factors.sort();
    for factor in factors {
        let mut a = 1;
        if check {
            if factor <= limit {
                let factor_log = factor.log10();
                if factor_log > 0 {
                    a = max.log10() / factor_log;
                }
            }else{
                check = false;
            }
        }
        num *= factor.pow(a);
    }



    num

}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_five(){
        assert_eq!(solution(5), 60);
    }
    #[test]
    fn test_six(){
        assert_eq!(solution(6), 60);
    }
    #[test]
    fn test_base(){
        assert_eq!(solution(10), 2520);
    }

}
