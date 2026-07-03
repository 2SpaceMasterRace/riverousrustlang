# Drop

Do we need to worry about cleaning up our list? Technically, no, not at all! Like C++, Rust uses destructors to automatically clean up resources when they're done with. A type has a destructor if it implements a trait called Drop. Traits are Rust's fancy term for interfaces. The Drop trait has the following interface:

pub trait Drop {
    fn drop(&mut self);
}

Basically, "when you go out of scope, I'll give you a second to clean up your affairs".

You don't actually need to implement Drop if you contain types that implement Drop, and all you'd want to do is call their destructors. In the case of List, all it would want to do is drop its head, which in turn would maybe try to drop a Box<Node>. All that's handled for us automatically... with one hitch.

The automatic handling is going to be bad.

Let's consider a simple list:

list -> A -> B -> C

When list gets dropped, it will try to drop A, which will try to drop B, which will try to drop C. Some of you might rightly be getting nervous. This is recursive code, and recursive code can blow the stack!

Some of you might be thinking "this is clearly tail recursive, and any decent language would ensure that such code wouldn't blow the stack". This is, in fact, incorrect! To see why, let's try to write what the compiler has to do, by manually implementing Drop for our List as the compiler would:

impl Drop for List {
    fn drop(&mut self) {
        // NOTE: you can't actually explicitly call `drop` in real Rust code;
        // we're pretending to be the compiler!
        self.head.drop(); // tail recursive - good!
    }
}

impl Drop for Link {
    fn drop(&mut self) {
        match *self {
            Link::Empty => {} // Done!
            Link::More(ref mut boxed_node) => {
                boxed_node.drop(); // tail recursive - good!
            }
        }
    }
}

impl Drop for Box<Node> {
    fn drop(&mut self) {
        self.ptr.drop(); // uh oh, not tail recursive!
        deallocate(self.ptr);
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        self.next.drop();
    }
}

We can't drop the contents of the Box after deallocating, so there's no way to drop in a tail-recursive manner! Instead we're going to have to manually write an iterative drop for List that hoists nodes out of their boxes.








Bonus Section For Premature Optimization!

Our implementation of drop is actually very similar to while let Some(_) = self.pop() { }, which is certainly simpler. How is it different, and what performance issues could result from it once we start generalizing our list to store things other than integers?
Click to expand for answer

Pop returns Option<i32>, while our implementation only manipulates Links (Box<Node>). So our implementation only moves around pointers to nodes, while the pop-based one will move around the values we stored in nodes. This could be very expensive if we generalize our list and someone uses it to store instances of VeryBigThingWithADropImpl (VBTWADI). Box is able to run the drop implementation of its contents in-place, so it doesn't suffer from this issue. Since VBTWADI is exactly the kind of thing that actually makes using a linked-list desirable over an array, behaving poorly on this case would be a bit of a disappointment.

If you wish to have the best of both implementations, you could add a new method, fn pop_node(&mut self) -> Link, from-which pop and drop can both be cleanly derived.

