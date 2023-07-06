fn main() {
    // _struct_type();

    // _struct_method();
}

struct MyString<'a> {
    text: &'a str
}

fn _lifetime_static() {
    // 悬空引用
    let str = String::from("My String");
    let x = MyString{text: &str.as_str()};
    let y: &'static str = "I have static lifetime";
}


fn _lifetime_struct() {
    // 悬空引用
    let str = String::from("My String");
    let x = MyString{text: &str.to_string()};
    
    let y = MyString{text: &str.as_str()};
}


fn _lifetime() {
    fn example<'a>(x:&'a str) -> &'a str {
        x
    }
}

struct Square{
    width:u32,
    height:u32,
}

impl Square {
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn _change_width(&mut self, new_width: u32) {
        self.width = new_width
    }

    fn whats_width(&self) -> u32 {
        self.width
    }
}


fn _struct_method() {
    let mut sq = Square{width:40, height: 50};
    println!("this area is {}", sq.area());

    println!("this old width is {}", sq.whats_width());
    sq._change_width(10);

    println!("this new width is {}", sq.whats_width());
}


fn _struct_type() {
    

    let user = User{active: true, username:"Liam".to_string(), sign_in_count:0};
    println!("username is {}", user.username);

    let user2 = _build_user("Amma".to_string());
    println!("user2's username is {}", user2.username);

    let coor = _build_coordinates();
    println!("{}", coor.1);
}
struct UnitUser;

struct User {
    active: bool,
    username:String,
    sign_in_count:u32,
}

struct Coordinates(i32, i32, i32);

fn _build_coordinates() -> Coordinates{
    Coordinates(1, 2, 3)
}

fn _build_user(username:String) -> User {
    User { 
        username, 
        active: true, 
        sign_in_count: 1 
    }
}