use std::collections::HashMap;
fn main() {
    _hash_map();
}

fn _hash_map() {
    let mut hm = HashMap::new();
    hm.insert(1, 1);
    hm.insert(5, 2);
    hm.insert(30,5);
    let old = hm.insert(30, 2); // 会返回旧的值
    println!("{:?}", hm);
    println!("{:?}", old);
    println!("{}", hm.contains_key(&30)); // 查询key是否存在
    println!("{}", hm.contains_key(&8));

    println!("{:?}", hm.get(&30)); // return Option<T>, Some(T)
    println!("{:?}", hm.get(&8));

    let one = hm.remove(&30); // 返回值
    println!("{:?}", hm);
    println!("{:?}", one);

    let remove = hm.remove_entry(&1); // 返回键值对
    println!("{:?}", remove);
    println!("{:?}", hm);

    hm.clear(); // 快速全部清除
    println!("{:?}", hm);

    let x = hm.remove(&5);
    println!("{:?}", x);
}