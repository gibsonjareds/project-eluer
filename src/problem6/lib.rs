
pub fn run(){
    println!("Problem 6: {}", solution(100));
}

fn solution(max:i64) -> i64{
    let mut square_sum:i64 = 0;
    let mut sum_square:i64 = 0;
    for i in 0..(max+1){
        sum_square += i.pow(2);
        square_sum += i;
    }
    square_sum = square_sum.pow(2);
    square_sum - sum_square
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_base(){
        assert_eq!(solution(10), 2640);
    }

}
