pub mod first;

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
