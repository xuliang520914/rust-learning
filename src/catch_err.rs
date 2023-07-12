use std::fs::File;
use std::io::ErrorKind;
fn main() {
    _panic_fn();
    // _catch_err();
    _catch_err2();
}

fn _panic_fn() {
    // panic!() // 终止主线程
    // panic!("panicked here");

    let _vec = vec![1]; // windows -> $env:RUST_BACKTRACE=1

    // vec[10];
}

fn _catch_err2() {
    // let _file = File::open("err.txt").unwrap();
    let _file = File::open("err.txt").expect("Error openng the file!");

}

fn _catch_err() {
    let _file = File::open("err.txt");
    let _file = match _file {
        Ok(_file) => _file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("err.txt") {
                Ok(file_create) => file_create,
                Err(e) => panic!("Can not create the file!: -- {:?}", e),
            },
            _ => panic!("It was some other error"),
        },
    };
}

//enum Result<T, E> {
//    Ok(T),
//    Err(E),
//}


