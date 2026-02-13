use std::rc::Rc;
use std::cell::RefCell;

// Двусвязный список
type Link = Option<Rc<RefCell<DNode>>>;

pub struct DNode {
    data: String,
    prev: Link,
    next: Link,
}

pub struct DoublyList {
    head: Link,
    tail: Link,
}

impl DoublyList {
    pub fn new() -> Self {
        DoublyList {
            head: None,
            tail: None,
        }
    }

    pub fn add_head(&mut self, value: String) {
        let new_node = Rc::new(RefCell::new(DNode {
            data: value,
            prev: None,
            next: None,
        }));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::clone(&new_node));
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
        }
    }

    pub fn add_tail(&mut self, value: String) {
        let new_node = Rc::new(RefCell::new(DNode {
            data: value,
            prev: None,
            next: None,
        }));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&new_node));
                new_node.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
        }
    }

    pub fn remove_head(&mut self) -> Result<(), String> {
        match self.head.take() {
            Some(old_head) => {
                match old_head.borrow_mut().next.take() {
                    Some(new_head) => {
                        new_head.borrow_mut().prev = None;
                        self.head = Some(new_head);
                    }
                    None => {
                        self.tail = None;
                    }
                }
                Ok(())
            }
            None => Err("List is empty".to_string()),
        }
    }

    pub fn remove_tail(&mut self) -> Result<(), String> {
        match self.tail.take() {
            Some(old_tail) => {
                match old_tail.borrow_mut().prev.take() {
                    Some(new_tail) => {
                        new_tail.borrow_mut().next = None;
                        self.tail = Some(new_tail);
                    }
                    None => {
                        self.head = None;
                    }
                }
                Ok(())
            }
            None => Err("List is empty".to_string()),
        }
    }

    pub fn find(&self, value: &str) -> bool {
        let mut current = self.head.clone();
        while let Some(node) = current {
            if node.borrow().data == value {
                return true;
            }
            current = node.borrow().next.clone();
        }
        false
    }

    pub fn print_forward(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            print!("{} ", node.borrow().data);
            current = node.borrow().next.clone();
        }
        println!();
    }

    pub fn print_backward(&self) {
        let mut current = self.tail.clone();
        while let Some(node) = current {
            print!("{} ", node.borrow().data);
            current = node.borrow().prev.clone();
        }
        println!();
    }

    pub fn iter(&self) -> DoublyListIter {
        DoublyListIter {
            current: self.head.clone(),
        }
    }
}

pub struct DoublyListIter {
    current: Link,
}

impl Iterator for DoublyListIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            let data = node.borrow().data.clone();
            self.current = node.borrow().next.clone();
            data
        })
    }
}