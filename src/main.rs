fn main() {

    //reborrow
    let mut a = vec![1, 2, 3];
    let b = &mut a;
    b.push(2);
    //{
    let c: &mut Vec<_> = b;
    c.push(5);
    //}
    b.push(4);
    println!("Hello, world!");
}
