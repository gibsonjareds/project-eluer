pub fn run(){
    println!("Problem 4: {}", solution(3));
}

fn solution(digits:u32) -> i64{
   let upper = 10_i32.pow(digits);
   let lower = upper/10 -1;
   let mut prod:i64 = 0;
   let mut found_answer = false;
   for a in  (lower..upper).rev(){
       for b in (lower..upper).rev() {
          prod = (a * b).into();
          
          if prod % 2 != 0 {
            let mut prod_str:String = format!("{}",prod);
            let str_a:String;
            if prod_str.len() % 2 == 0 {
                  str_a = prod_str.split_off(prod_str.len() / 2); 
            }else{
                  str_a  = prod_str.split_off((prod_str.len() - 1) /2);
                  prod_str.pop();
            }
            if str_a.chars().rev().collect::<String>() == prod_str{
                  found_answer = true;
                  break;
            }
          }
          if b < a - 10{
              break;
          }
       }
       if found_answer{
           break;
       }
   }

   prod
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_base(){
        assert_eq!(solution(2), 9009);
    }

}
