extern crate utils;



pub fn run(){
    println!("Problem 10: {}", solution(2000000));
}

fn solution(max:i64) -> i64{
    if max < 2{
        return 0;
    }
    if max == 2 {
        return 2;
    }
    let mut sum:i64 = 2;
    for i in (3..(max as usize)).step_by(2) {
        if utils::is_prime(i as i64){
            sum += i as i64;
        }
    }

    sum
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_base(){
        assert_eq!(solution(10), 17);
    }

}
