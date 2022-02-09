extern crate utils;



pub fn run(){
    println!("Problem 9: {}", solution(1000));
}

fn solution(sum:i64) -> i64{
    let mut a:i64 = 1;
    let mut b:i64 = 2;
    let mut c:i64 = 3;
    let mut found = false;
    let maybe_c = sum / 2;

    for i in 0..(maybe_c as usize) {
        if found {
            break;
        }
        c = maybe_c - i as i64;
        let maybe_b = sum - maybe_c;
        for e in 0..(maybe_b as usize) {
            b = maybe_b - e as i64;
            a = sum - b - c;
            if a >= b || b >= c {
                continue;
            }
            if a.pow(2) + b.pow(2) == c.pow(2){
                found = true;
                break;
            }
        }
    }


    a * b * c
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_base(){
        assert_eq!(solution(12), 60);
    }

}
