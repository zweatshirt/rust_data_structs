/* Stacks follow the last in first out 
*  principle. (LIFO)
*  - utilizes push and pop like any other stack
*/

/* Queues follow the first in last out principle
* (FILO)
* - utilizes enqueue and dequeue like any other queue
*/

use std::collections::VecDeque;

struct Stack {
    items: Vec<i32>,
}

impl Stack {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    // push to top of stack
    fn push(&mut self, value: i32) {
        self.items.push(value);
    }

    // pop off stack
    fn pop(&mut self) -> Option<i32> {
        self.items.pop()
    }

    // see what is next to be popped
    fn peek(&self) -> Option<&i32> {
        self.items.last()
    }
}

struct Queue {
    items: VecDeque<i32>
}

impl Queue {
    fn new() -> Self {
        Queue { items: VecDeque::new() }
    }

    fn enqueue(&mut self, value: i32) {
        self.items.push_back(value);
    }

    fn dequeue(&mut self) -> Option<i32> {
        self.items.pop_front()
    }

    fn peek(&self) -> Option<&i32> {
        self.items.front()
    }
}

pub fn stacks() {
    let mut stack = Stack::new();
    stack.push(30); 
    println!("{:?}", stack.peek().unwrap());
    for i in 0..10 {
        stack.push(i);
    }
    println!("{:?}", stack.peek());
}  

pub fn queues() {
    let mut queue = Queue::new();
    for i in (0..21).step_by(2) {
        queue.enqueue(i);
    }
    println!("{:?}", queue.items);
}
