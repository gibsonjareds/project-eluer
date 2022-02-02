pub fn run(){
    println!("Problem 1: {}", solution(1000));
}

fn solution(bound: u32) -> u32 {
    let mut sum:u32 = 0;
    for i in 1..bound{
        if i % 3 == 0{
            sum += i
        }else if i % 5 == 0{
            sum += i
        }
    }
    sum
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_base(){
        assert_eq!(solution(10), 23);
    }

}
