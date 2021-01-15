pub struct List {
    head: Option<Box<Node>>,
}

struct Node {
    elem: i32,
    next: Option<Box<Node>>,
}
