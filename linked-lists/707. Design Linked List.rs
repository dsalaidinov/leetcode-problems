Link: https://leetcode.com/problems/design-linked-list/


struct Node {
    value: i32,
    next: Option<Box<Node>>
}

struct MyLinkedList {
    head: Option<Box<Node>>,
    size: usize
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {

    fn new() -> Self {
        MyLinkedList {
            head: None,
            size: 0
        }
    }
    
    fn get(&self, index: i32) -> i32 {
        if index < 0 || index as usize >= self.size {
            return -1; 
        }

        let mut current = &self.head;

        for _ in 0..index {
            if let Some(node) = current {
                current = &node.next;
            }
        }

        if let Some(node) = current {
                node.value
            } else {
                -1
            }
    }
    
    fn add_at_head(&mut self, val: i32) {
        self.add_at_index(0, val)
    }
    
    fn add_at_tail(&mut self, val: i32) {
        self.add_at_index(self.size as i32, val)
    }
    
    fn add_at_index(&mut self, index: i32, val: i32) {
        if index < 0 || index > self.size as i32 {
            return;
        }

        let new_node = Box::new(Node {
            value: val,
            next: None
        });

        if index == 0 {
            let old_head = self.head.take();
            self.head = Some(new_node);

            if let Some(ref mut node) = self.head {
                node.next = old_head;
            }
        } else {
             let mut current = self.head.as_mut();

            for _ in 0..index - 1 {
                if let Some(node) = current {
                    current = node.next.as_mut();
                }
            }

            if let Some(node) = current {
                let old_next = node.next.take();
                node.next = Some(new_node);
                node.next.as_mut().unwrap().next = old_next;
            }
        }

        self.size += 1;
    }
    
    fn delete_at_index(&mut self, index: i32) {
        if index < 0 || index >= self.size as i32 {
            return;
        }

        if index == 0 {
            self.head = self.head.take().and_then(|node| node.next);
        } else {
             let mut current = self.head.as_mut();

        for _ in 0..index-1 {
            if let Some(node) = current {
                current = node.next.as_mut()
            }
        }

        if let Some(node) = current {
            node.next = node.next.take().and_then(|node| node.next);
        }

        }

       
        self.size -= 1;
    }
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */