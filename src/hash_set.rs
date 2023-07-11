use std::collections::HashSet;
fn main() {
    _hash_set();
}

fn _hash_set() {
    let mut hs = HashSet::new();
    // is_empty()、len()
    hs.insert(1);
    hs.insert(2);
    hs.insert(3);
    hs.insert(4);

    println!("{:?}", hs);
    for x in hs.iter() {
        println!("x is {}", x);
    }
    // hs.remove(&2);
    // println!("{:?}", hs);

    let mut hs1 = HashSet::new();
    hs1.insert(1);
    hs1.insert(2);
    hs1.insert(5);
    hs1.insert(7);

    // 交集
    for x in hs.intersection(&hs1) {
        println!("Intersection: {}", x);
    }
    let intersection = &hs & &hs1;
    println!("Intersection: {:?}", intersection);
    for x in intersection {
        println!("Intersection: {}", x);
    }

    let union = &hs | &hs1;
    println!("{:?}", union);

    let union1 = hs.union(&hs1);
    println!("union 1 :{:?}", union1);

    let diff = hs.difference(&hs1);
    println!("difference: {:?}", diff);

    let diff1 = hs1.difference(&hs);
    println!("difference 1: {:?}", diff1);

    let diff2 = &hs - &hs1;
    println!("difference 2: {:?}", diff2);

}