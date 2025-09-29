pub mod first;

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
}

struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Nodes>),
}

struct Nodes {
    elem: i32,
    next: Link,
}
