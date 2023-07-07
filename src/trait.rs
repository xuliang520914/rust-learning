use std::ops::Add;

fn main() {
    let course1 = Course{headline:String::from("Headeline!"), author:String::from("Steven")};
    let course2 = AnotherCourse{headline:String::from("Another Headeline!"), author:String::from("Another Steven")};

    call_overview(&course1);
    // println!("{}", course1.overview());
    // println!("{}", course2.overview());
    call_overview_generics(&course2);

    // Drop
    // drop(course1);

    let coord1 = Point{x:5.0, y:5.0};
    let coord2 = Point{x:1.0, y:2.0};

    let sum = coord1 + coord2;
    println!("{:?}", sum);
}

trait Overview {
    fn overview(&self) -> String {
        String::from("This is rust course")
    }
}

struct Course{
    headline: String,
    author: String,
}

struct AnotherCourse{
    headline: String,
    author: String,
}

impl Overview for AnotherCourse {
    fn overview(&self) -> String {
        format!("{} {}", self.author, self.headline)
    }
}

impl Overview for Course {
    // fn overview(&self) -> String {
    //     format!("{} {}", self.author, self.headline)
    // }
}


// traits as parameters
fn call_overview(item: &impl Overview){
    println!("Overview: {}", item.overview())
}

fn call_overview_generics<T: Overview>(item: &T){
    println!("Overview: {}", item.overview())
}
// fn overview(item1: &impl Overview, item2: &impl Overview) // 参数可以是不同类型
// fn overview<T: Overview>(item1, item2) // 参数必须是同一种类型
// fn overview(item1: &impl Overview + AnotherTrait) // 多个trait
// fn overview<T: Overview + AnotherTrait>(item1: &T, item2: &T)

// Drop
impl Drop for Course{
    fn drop(&mut self) {
        println!("Dropping:{}", self.author);
    }
}

trait Clone : Sized{
    fn clone(&self) -> Self;

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

// 两种复制方法 derive/clone

#[derive(Debug)]
struct Point<T>{
    x: T,
    y: T,
}

impl <T> Add for Point<T> 
    where
    T: Add<Output=T>{
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Point { 
                x: self.x + rhs.x,
                y:self.y+ rhs.y
            }
        }
    }