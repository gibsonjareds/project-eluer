extern crate utils;

pub fn run() {
    let answer = solution(1000000);
    println!("Problem 14: {}", answer.1);
}

fn solution(limit: u64) -> (u64, u64) {
    let mut sequence_length = 0;
    let mut longest_sequence = (0, 0);
    let mut n = limit / 2;
    while n < limit {
        sequence_length = 0;
        let mut n_increment = n;
        while n_increment > 1 {
            if n_increment % 2 == 0 {
                n_increment = n_increment / 2;
            } else {
                n_increment = n_increment * 3 + 1;
            }
            sequence_length += 1;
        }
        if sequence_length > longest_sequence.0 {
            longest_sequence.0 = sequence_length;
            longest_sequence.1 = n;
        }
        n += 1;
    }
    longest_sequence
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sequences() {
        assert_eq!(solution(13), (19, 9));
    }
}
