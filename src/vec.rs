use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    _collection();
}

fn _collection() {
    let mut nums: Vec<i32> = vec![];
    nums.push(1);
    nums.push(2);
    nums.push(3);

    let pop_num = nums.pop(); // Option<T>, return None or Some(T)
    println!("{:?}", pop_num);

    let two = nums[1]; // copy
    // &nums[1], creates a reference if copy is not avarilable
    println!("{}", two);

    println!("{:?}", nums);
    let one = nums.first();
    println!("{:?}", one);

    let last = nums.last();
    println!("{:?}", last);

    let one_mut = nums.first_mut(); //so will borrow mutable references
    println!("{:?}", one_mut);
    println!("{}", nums[0]);

    let last_mut = nums.last_mut();
    println!("{:?}", last_mut);

    println!("{}", nums.len());
    println!("{}", nums.is_empty());

    nums.insert(0, 10);
    nums.insert(3, 18);
    nums.insert(2, 25);
    println!("{:?}", nums);

    nums.remove(2);
    println!("{:?}", nums);
    
    nums.reverse();
    println!("{:?}", nums);

    nums.sort();
    println!("{:?}", nums);

    nums.reverse();
    println!("{:?}", nums);

    nums.shuffle(&mut thread_rng());
    println!("{:?}", nums);
}