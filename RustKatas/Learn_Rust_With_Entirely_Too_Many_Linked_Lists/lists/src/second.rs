pub struct List<T> {
    head: Link<T>,
}

pub trait Drop {
    fn drop(&mut self);
}

// type alias are the coolest thing ever !!!
type Link<T> = Option<Box<Node<T>>>;

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
// Tuple Structs
pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // not comfortable "hiding" that a struct contains a lifetime, so we use the  Rust 2018 "explicitly elided lifetime" syntax, '_:
    pub fn iter<'a>(&'a self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    // derive pop and drop cleanly
    pub fn pop_node(&mut self) -> Link<T> {
        //TODO
        unimplemented!()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();
        // `while let` == "do this thing until this pattern doesn't match"
        // boxed_node goes out of scope and gets dropped here;
        // but its Node's `next` field has been set to None
        // so no unbounded recursion occurs.
        while let Some(mut boxed_node) = current_link {
            current_link = boxed_node.next.take();
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access tuple numerically
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref().map::<&Node<T>, _>(|node| &node);
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use crate::second::Iterator;

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

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
