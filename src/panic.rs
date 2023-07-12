fn main() {
    _panic_fn();
}

fn _panic_fn() {
    // panic!() // 终止主线程
    // panic!("panicked here");

    let vec = vec![1]; // windows -> $env:RUST_BACKTRACE=1

    vec[10];
}
