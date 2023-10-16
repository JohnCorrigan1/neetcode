struct MinStack {
    min_element: Vec<i32>,
    stack: Vec<i32>,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            min_element: vec![],
            stack: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.min_element.len() == 0 {
            self.min_element.push(val);
        } else {
            if val < self.min_element[self.min_element.len() - 1] {
                self.min_element.push(val);
            }
        }
    }

    fn pop(&mut self) {
        let val = self.stack.pop();
        if val == Some(self.min_element[self.min_element.len() - 1]) {
            self.min_element.pop();
        }
    }

    fn top(&self) -> i32 {
        self.stack[self.stack.len() - 1]
    }

    fn get_min(&self) -> i32 {
        self.min_element[self.min_element.len() - 1]
    }
}

/*
 Your MinStack object will be instantiated and called as such:
 let obj = MinStack::new();
 obj.push(val);
 obj.pop();
 let ret_3: i32 = obj.top();
 let ret_4: i32 = obj.get_min();
*/

fn main() {
    let mut obj = MinStack::new();

    obj.push(100);
    obj.push(2);
    obj.push(-5);
    obj.pop();
    println!("{}", obj.get_min());
    println!("{}", obj.top());
}
