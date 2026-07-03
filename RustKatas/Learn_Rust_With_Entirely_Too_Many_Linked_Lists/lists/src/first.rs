pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: List,
}

impl List {
    pub fn new() -> self {
        List { head: Link::Empty }
    }
}

