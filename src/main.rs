use std::time::Instant;

fn main() {
    let start = Instant::now();
    run();
    println!("Completed in {:?}", start.elapsed());
}
fn bench(f: &dyn Fn() -> ()) {
    let start = Instant::now();
    f();
    println!("run {:?}", start.elapsed());
    println!("===================================");
    println!("===================================");
}
fn run() {
    println!("===================================");
    println!("===================================");
    bench(&problem1::run);
    bench(&problem2::run);
    bench(&problem3::run);
    bench(&problem4::run);
    bench(&problem5::run);
    bench(&problem6::run);
    bench(&problem7::run);
    bench(&problem8::run);
    bench(&problem9::run);
    bench(&problem10::run);
    bench(&problem11::run);
    bench(&problem12::run);
}
