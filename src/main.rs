mod lib;
use lib::queue::*;
fn main() {
    let mut queue = Queue::<i32>::new();
    queue.add(1);
    let peek = match queue.peek() {
        Ok(x) => x,
        Err(_e) => 0,
    };
    let res = match queue.remove() {
        Ok(res) => res,
        Err(_e) => 0
    };
}
