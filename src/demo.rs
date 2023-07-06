// fn main() {
//     let mut x = 1; // 要使得变量可变 前面加上mut
//     println!("{}", x);
//     x = 42;
//     println!("{}", x);
// }

fn main() {
    // 数据类型
    // data_type();

    // 运算
    // operator();

    // 复合数据类型
    // compound_type();
    
    // 切片 slice 对数组/向量
    // slice();
    
    // 字符串
    // string();
    
    // 函数
    // print_phase("print my argument");
    // let x = gcd(15, 8);
    // println!("{}", x);
    
    // println!("{}", multiple_return_values(true));

    // 程序控制
    // contr();
    
    // ownership
    // _ownership();
    
    // take ownership
    // let s = String::from("hello world");
    // _takes_ownership(s);
    // println!("{}", s); // 这里也是 move
    
    // let i = 12;
    // _make_copy(i);
    // println!("i is {}", i);
    
    // let str = _give_ownership();
    // println!("str is {}", str);
    
    // let str1 = _take_and_give(str);
    // println!("str is {}", str); // 这里也是 move
    // println!("str1 is {}", str1);

    let mut str = "Hello".to_string();
    _change_string(&mut str);
    println!("str is {}", str);

    let mut str1 = String::from("Hello 1");
    _change_string(&mut str1);
    println!("str1 is {}", str1);
}

// 引用和传递
fn _change_string(str: &mut String) {
    str.push_str(", world");
    println!("{}", str);
}

fn _take_and_give(str: String) -> String {
    str
}

fn _give_ownership() -> String {
    "give ownership".to_string()
}

fn _make_copy(i: i32) {
    let ii = i;
    println!("ii is {}", ii)
}

fn _takes_ownership(s: String){
    let str = s;
    println!("{}", str);
}

fn _ownership() {
    {
        let _var=1; // on the stack 栈
        let mut s = "hello".to_string(); // on heap 堆
        
        s.push_str(", world");
        
        // move
        let x = vec!["hello".to_string()];
        let y = x;
        // println!("{:?}", x);// 发生了move,会报error: value borrowed here after move
        println!("y is {:?}", y);
        
        // clone
        let w = y.clone();
        println!("y is {:?}", y);

        println!("w is {:?}", w);
    }
    // var is dropped, s is dropped
}

fn _contr() {
    // 判断
    let one = 1;
    if one > 10 {
        println!("True");
    } else if one == 1 {
        println!("Equal");
    } else {
        println!("False");
    }

    // 循环
    let mut x = 3;
    while x > 0 {
        println!("{}", x);
        x -= 1;
    }
    //    loop {
    //        println!("loop"); // 死循环
    //    }
    println!("====");
    let mut num = 0;
    'counter: loop{
        println!("counter");
        let mut decrease = 5;
        loop {
            println!("decrease is {}", decrease);
            if decrease == 3 {
                break;
            }
            println!("num is {}", num);
            if num == 2 {
                break 'counter;
            }
            decrease -= 1;
        }
        num += 1;
    }

    // for 循环集合
    let vec: Vec<i8> = (0..10).collect();
    for v in vec {
        println!("v is {}", v);
    }

    for v in (0..4).rev() {
        println!("{}", v);
    }
    println!("LIFE OFF");
}

fn _multiple_return_values(flag: bool) -> bool {
    flag
}

fn _gcd(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        if a < b {
            let c = a;
            a=b;
            b=c;
        }
        a = a % b;
    }
//    return b;
    b
}

fn _print_phase(phase: &str){
    println!("{}", phase);
}

fn _ddd() {
    _string();
}

fn _string() {
    // 字符串与向量相似,因为它存储为字节向量
    // 不同的是,字符串保证始终是有效的utf8序列
    // 堆上分配一个字符串,它是grovel并且不是以null结尾
    // String::from()
    let str1 = String::from("steven xu");
    println!("{}", str1);
    let str2 = "steven xu".to_string();
    println!("{}", str2);
    let str3 = str1.replace("steven", "linus");
    println!("{}", str3);
    // &str - "string slice" or "stir"
    let str4 = "steven";
    println!("{}", str4); // &str
    _print_type_of(&str4);
    let str5 = str4.to_string();
    println!("{}", str5);
    _print_type_of(&str5);
    let str6 = &str5;
    println!("{}", str6);
    _print_type_of(&str6);
    println!("{}", "ONE".to_lowercase() == "one");
    println!("{}", "ONE".to_lowercase().to_string() == "one");
    println!("string end")
}

fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn _slice(){
    let v: Vec<i32> = (0..5).collect();
    println!("vec is {:?}", v);
    let sv: &[i32] = &v; // 双切片的引用称为 "胖指针", 它包含两个值 - 切片的第一个元素(地址)和元素个数
    println!("slice vec is {:?}", sv);
    
    let sv1: &[i32] = &v[1..4];
    println!("sv1 is {:?}", sv1);
    // 普通引用时非拥有指针指向单个值, 但对切片的引用是指向一系列连续值的非投票指针
    // 同样普通引用指向一个单一的值，切片指向一段连续的范围值，两者都是非编码
    // 切片引用的使用场景 - 如果你想编写一个对数组或向量进行操作的函数时使用切片
    
    println!("slice end");
}

// 复合数据类型
fn _compound_type(){
    // 元组
    let tup = (4, 4.2, "hello", false); // 声明后不能更改
    println!("{} ", tup.1);

    let (x, y, z, w) = tup;
    println!("{} {} {} {}", x, y, z, w);

    println!("compound type end === ");

    // 数组
    let mut arr1 = [1,2,3];
    let _arr2:[i32; 4] = [4,5,6,7]; // [数据类型; 长度]
    println!("{}", arr1[2]);

    arr1[2] = 10;
    println!("{}", arr1[2]);

    println!("{:?}", arr1);

    // 向量 vector
    // 宏定义
    let mut nums = vec![1, 2,3];
    nums.push(4);
    println!("{}", nums[3]);

    println!("{:?}", nums);
    nums.pop();
    println!("{:?}", nums);
    
    // new
    let mut vec = Vec::new(); // vec!
    vec.push(2);
    println!("{:?}", vec);
    
    let mut vect = Vec::<i32>::with_capacity(2);
    println!("{:?}", vect.capacity());
    vect.push(3);
    println!("{:?}", vect);
    let vec1: Vec<i32> = (1..5).collect(); // 左边包含右边不包含 - 左开右闭
    println!("{:?}", vec1);
    
    println!("operator end ");
}

// 运算符
fn _operator(){
    let a = 11;
    let b = 4;
    println!("{} ", (a+b));
    println!("{} ", (a-b));
    println!("{} ", (a*b));
    println!("{} ", (a/b));
    println!("{} ", (a%b));

    println!("compound type end ");
}

// 数据类型
fn _data_type() {
    let x:u8 = 10;
    println!("{}", x);

    let mut d = 02_55; 
    d = d + 3;
    println!("{} ", d);

    println!("d == 258 is {} ", (d == 258));

    let hex = 0xff;
    println!("{} ", hex);

    let octal = 0o377;
    println!("{} ", octal);
    println!("hex == octal is {} ", (hex == octal));

    let binary = 0b1111_1111;
    println!("binary is {} ", binary);

    let byte = b'X';
    println!("{} ", byte);

    let _no_use = ""; // 没有使用的变量 前面加下划线，不会报错
    

    // 浮点数
    let f64 = 2.0; // 默认f64
    println!("{} ", f64);
    let f32:f32 = 2.1;
    println!("{} ", f32);

    // 布尔
    let t = true;
    let f:bool = false;
    println!("{} {} ", t, f);

    // char
    let c = 'c';
    println!("{} ", c);

    println!("data type end ");
}
