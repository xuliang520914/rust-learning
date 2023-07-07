fn main() {
    // _some_number();

    // _match_pet();
    

    // _what_pet("Dog");
    // _what_pet("Cat");
    // _what_pet("Cow");
    // _if_match();
    _more_matches();
}

fn _more_matches(){
    let i = 13;
    match i {
        1|2 => println!("one or two"),
        _ => println!("not one or two"),
    }

    match i {
        1..=8 => println!("matches"),
        _ => println!("not matching"),
    }

    println!("======");
    let x = Some(10);
    let y = 5;
    match x {
        Some(10) => println!("matches"),
        Some(x) if x == y => println!("x == y"),
        _ => println!("default")
    }
}

fn _if_match() {
    let dog = Some(Pet::dog);
    // 变量在后面
    if let Some(Pet::dog) = dog {
        println!("i have a dog");
    } else {
        println!("it is not dog");
    }

    let mut stact = Vec::new();
    stact.push(1);
    stact.push(2);
    stact.push(3);

    while let Some(top) = stact.pop() {
        println!("{}", top);
    }
}


// enum Option<T> {
//     None,
//     Some(T)
// }
fn _some_number() {
    let some_number = Some(5);
    let some_number1 = Some(8);
    let some_string = Some("string");
    let nothing: Option<i32> = None;
    let x: i32 = 3;
    // let y = some_number + some_number1;  
    // let y = some_number + x;
    // 上面两个都会报错 

    let six = _match_option(some_number);
    println!("{:?}", six);
    let none = _match_option(None);
    println!("{:?}", none);
}

fn _match_option(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn _what_pet(input: &str) {
    match input {
        "Dog" => println!("I have a dog"),
        "Cat" => println!("I have a cat"),
        "Fish" => println!("I have a fish"),
        _ => println!("I have no clue what pet you have"),
    }
}

enum IpAddrKind {
    V4,
    V6
}

struct IpAddr {
    kind:IpAddrKind,
    address:String
}

fn enum_ip_addr() {
    let home = IpAddrKind::V4;

    let loopack = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
}


enum Pet {
    dog,
    cat,
    fish
}

impl Pet {
    fn what_am_i(self) -> &'static str{
        match self {
            Pet::cat => "I am a cat",
            Pet::fish => "I am a fish",
            Pet::dog => "I am a dog",
        }
    }
}

fn _match_pet() {
    let dog = Pet::dog;
    println!("{}", dog.what_am_i());
}