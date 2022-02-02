pub fn run(){
    println!("Problem 2: {}", solution(4000000));
}
fn solution(bound:u32) -> u32{
    let mut sum:u32 = 0;
    let mut fib = vec![1,1];
    while fib[1] < bound {
        let a = fib[0];
        let b = fib[1];
        fib[0] = b;
        fib[1] = a+b;
        if fib[1] % 2 == 0 {
            sum += fib[1];
        }
    }
    sum
    
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_base(){
        assert_eq!(solution(10), 10);
    }

}
