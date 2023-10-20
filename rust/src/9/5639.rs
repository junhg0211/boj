use std::io::stdin;
use std::fmt::Display;
use std::cmp::PartialOrd;

struct BST<T> {
    value: Option<T>,
    left: Option<Box<BST<T>>>,
    right: Option<Box<BST<T>>>,
}

impl<T: Display + PartialOrd> BST<T> {
    fn new() -> Self {
        BST { value: None, left: None, right: None }
    }

    fn postorder(&self) {
        // left
        match &self.left {
            Some(thing) => thing.postorder(),
            None => (),
        }

        // right
        match &self.right {
            Some(thing) => thing.postorder(),
            None => (),
        }


        // self
        match &self.value {
            Some(thing) => println!("{}", thing),
            None => (),
        }
    }

    fn insert_value(&mut self, value: T) {
        if self.value.is_none() {
            self.value = Some(value);
            return;
        }

        let self_value = self.value.as_ref().unwrap();

        if &value < self_value {
            if self.left.is_none() {
                self.left = Some(Box::new(BST::new()));
            }

            self.left.as_mut().unwrap().insert_value(value);
        } else {
            if self.right.is_none() {
                self.right = Some(Box::new(BST::new()));
            }

            self.right.as_mut().unwrap().insert_value(value);
        }
    }
}

fn main() {
    let mut bst = BST::new();

    loop {
        let mut input = String::new();
        let bytes = stdin().read_line(&mut input).unwrap();

        if bytes == 0 {
            break;
        }

        let number = input.trim().parse::<u32>().unwrap();
        bst.insert_value(number);
    }

    bst.postorder();
}
