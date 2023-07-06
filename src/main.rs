fn main() {
    _match_pet();
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