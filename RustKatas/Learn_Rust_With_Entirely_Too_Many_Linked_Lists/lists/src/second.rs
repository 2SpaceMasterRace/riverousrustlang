pub struct List {
    head: Link,
}

// type alias are the coolest thing ever !!!
type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: std::mem::replace(&mut self.head, None),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

    // derive pop and drop cleanly
    pub fn pop_node(&mut self) -> Link {
        //TODO
        unimplemented!()
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut current_link = std::mem::replace(&mut self.head, None);
        // `while let` == "do this thing until this pattern doesn't match"
        // boxed_node goes out of scope and gets dropped here;
        // but its Node's `next` field has been set to None
        // so no unbounded recursion occurs.
        while let Some(mut boxed_node) = current_link {
            current_link = std::mem::replace(&mut boxed_node.next, None)
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // check the property of list when Empty
        assert_eq!(list.pop(), None);

        // populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // assert property of popping elements of the list
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // re-assess the list to avoid corrupution of data
        list.push(4);
        list.push(5);

        // pop elements again
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // check exhaustion

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
