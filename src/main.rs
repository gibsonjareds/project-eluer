use std::time::Instant;


fn main() {
    let start = Instant::now();
    run();
    println!("Completed in {:?}", start.elapsed());
}
fn run(){
    problem1::run();
    problem2::run();
    problem3::run();
    problem4::run();
    problem5::run();
    problem6::run();
    problem7::run();
    problem8::run();
    problem9::run();
    problem10::run();
    problem11::run();
    problem12::run();
}

