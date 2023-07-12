use std::fs::rename;
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
fn main() {
    // let test = _open_file();
    // test.unwrap();
    let file = _open_file("error.txt".to_string());
    match file {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match _create_file("error.txt".to_string()) {
                Ok(cf) => cf,
                Err(_err) => panic!("Don't create"),
            },
            _ => panic!("other error {}", e),
        },
    };
    _rename_file("rename.txt".to_string(), "error.txt".to_string()).unwrap();
}

fn _open_file(filename: String) -> Result<File, Error> {
    let file = File::open(filename)?;
    Ok(file)
}

fn _create_file(filename: String) -> Result<File, Error> {
    let file = File::create(filename)?;
    Ok(file)
}

fn _rename_file(filename: String, from: String) -> Result<(), Error> {
    let file = rename(from, filename)?;
    Ok(file)
}
