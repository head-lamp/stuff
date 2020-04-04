mod lib;
use lib::queue::*;
fn main() {
    let mut queue = Queue::<i32>::new(16);
    queue.add(3);
    queue.add(2);
    queue.add(1);
    let peek = match queue.peek() {
        Ok(x) => x,
        Err(_e) => 0,
    };
    println!("peek = {:?}", peek);
    let res = match queue.remove() {
        Ok(res) => res,
        Err(_e) => 0
    };
    println!("res = {:?}", res);
    let peek = match queue.peek() {
        Ok(x) => x,
        Err(_e) => 0,
    };
    println!("peek = {:?}", peek);
    let res = match queue.remove() {
        Ok(res) => res,
        Err(_e) => 0
    };
    println!("res = {:?}", res);
    let peek = match queue.peek() {
        Ok(x) => x,
        Err(_e) => 0,
    };
    println!("peek = {:?}", peek);
    let res = match queue.remove() {
        Ok(res) => res,
        Err(_e) => 0
    };
    println!("res = {:?}", res);
}
