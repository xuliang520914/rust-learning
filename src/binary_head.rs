use std::collections::BinaryHeap;

fn main() {
    _binary();
}

fn _binary() {
    // 常用方法 push/pop/peak
    let mut bheap = BinaryHeap::new();
    bheap.push(1);
    bheap.push(18);
    bheap.push(51);
    bheap.push(24);
    bheap.push(80);

    println!("{:?}", bheap);
    println!("{:?}", bheap.pop());
    println!("{:?}", bheap);
    println!("{:?}", bheap.peek()); // 与pop相似得到一个返回值，但是不会删除 Some<T>
    println!("{:?}", bheap);

}