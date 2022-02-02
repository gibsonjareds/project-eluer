pub fn run(){
    println!("Problem 3: {}", solution(600851475143));
}

fn solution(num:i64) -> i64{
    let mut factors = vec![];
    let mut num2 = num;

        while num2 % 2 == 0 {
            num2 = num2 / 2;
            factors.push(2)
        }
        for i in (3..((num2 as f64).sqrt() as i64)).step_by(2){
            while num2 % i == 0{
                num2 = num2 / i;
                factors.push(i)
            }
        }

        if num2 > 2  {
            factors.push(num2)
        }
    

    factors.iter().fold(i64::MIN, |a, &b| a.max(b))
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_base(){
        assert_eq!(solution(13195), 29);
    }

}
