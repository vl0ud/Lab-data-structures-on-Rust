// Односвязный список
pub struct Node {
    data: String,
    next: Option<Box<Node>>,
}

pub struct SinglyList {
    head: Option<Box<Node>>,
    tail: *mut Node,
    size: usize,
}

impl SinglyList {
    pub fn new() -> Self {
        SinglyList {
            head: None,
            tail: std::ptr::null_mut(),
            size: 0,
        }
    }

    pub fn add_head(&mut self, value: String) {
        let mut new_node = Box::new(Node {
            data: value,
            next: self.head.take(),
        });

        let raw_node: *mut _ = &mut *new_node;

        if self.tail.is_null() {
            self.tail = raw_node;
        }

        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn add_tail(&mut self, value: String) {
        let new_node = Box::new(Node {
            data: value,
            next: None,
        });

        let raw_node: *mut _ = Box::into_raw(new_node);

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(Box::from_raw(raw_node));
            }
        } else {
            self.head = Some(unsafe { Box::from_raw(raw_node) });
        }

        self.tail = raw_node;
        self.size += 1;
    }

    pub fn remove_head(&mut self) -> Result<(), String> {
        if self.head.is_none() {
            return Err("List is empty".to_string());
        }

        self.head = self.head.as_mut().and_then(|node| node.next.take());

        if self.head.is_none() {
            self.tail = std::ptr::null_mut();
        }

        self.size -= 1;
        Ok(())
    }

    pub fn remove_tail(&mut self) -> Result<(), String> {
        if self.head.is_none() {
            return Err("List is empty".to_string());
        }

        if self.head.as_ref().unwrap().next.is_none() {
            self.head = None;
            self.tail = std::ptr::null_mut();
            self.size -= 1;
            return Ok(());
        }

        let mut current = self.head.as_mut().unwrap();
        while current.next.as_ref().unwrap().next.is_some() {
            current = current.next.as_mut().unwrap();
        }

        current.next = None;
        self.tail = current as *mut Node;
        self.size -= 1;
        Ok(())
    }

    pub fn remove_value(&mut self, value: &str) -> Result<(), String> {
        if self.head.is_none() {
            return Err("List is empty".to_string());
        }

        if self.head.as_ref().unwrap().data == value {
            return self.remove_head();
        }

        let mut current = self.head.as_mut().unwrap();
        while let Some(ref mut next_node) = current.next {
            if next_node.data == value {
                let removed = next_node.next.take();
                current.next = removed;
                self.size -= 1;
                return Ok(());
            }
            current = current.next.as_mut().unwrap();
        }

        Err("Value not found".to_string())
    }

    pub fn find(&self, value: &str) -> Option<usize> {
        let mut current = self.head.as_ref();
        let mut index = 0;

        while let Some(node) = current {
            if node.data == value {
                return Some(index);
            }
            current = node.next.as_ref();
            index += 1;
        }

        None
    }

    pub fn print(&self) {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            print!("{} -> ", node.data);
            current = node.next.as_ref();
        }
        println!("nullptr");
    }

    pub fn iter(&self) -> SinglyListIter {
        SinglyListIter {
            current: self.head.as_deref(),
        }
    }
}

pub struct SinglyListIter<'a> {
    current: Option<&'a Node>,
}

impl<'a> Iterator for SinglyListIter<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.data
        })
    }
}