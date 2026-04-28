fn main() {
    let mut stack = Stack::new();
    stack.push(42);
    stack.push(100);

    if let Some(val) = stack.pop() {
        println!("Popped: {}", val);
    }
}


struct Stack {
    data: [i32; 256],  // fixed-size array of 256 elements
    top: usize,        // points to next free slot
}

impl Stack {
    fn new() -> Self {
        Stack {
            data: [0; 256],
            top: 0,
        }
    }

    fn push(&mut self, element: i32) {
        if self.top < 256 {
            self.data[self.top] = element;
            self.top += 1;
        } else {
            println!("Stack overflow");
        }
    }

    fn pop(&mut self) -> Option<i32> {
        if self.top == 0 {
            None
        } else {
            self.top -= 1;
            Some(self.data[self.top])
        }
    }
}

