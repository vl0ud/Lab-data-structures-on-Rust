// Стек на основе вектора
pub struct Stack {
    data: Vec<String>,
}

impl Stack {
    pub fn new(capacity: usize) -> Self {
        Stack {
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, value: String) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Result<String, String> {
        self.data.pop().ok_or_else(|| "Stack is empty".to_string())
    }

    pub fn peek(&self) -> Result<&String, String> {
        self.data.last().ok_or_else(|| "Stack is empty".to_string())
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.data.iter().rev()
    }
}