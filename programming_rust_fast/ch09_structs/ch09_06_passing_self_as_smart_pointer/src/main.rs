use std::rc::Rc;

fn main() {}

struct Node {
    tag: String,
    children: Vec<Rc<Node>>
}

impl Node {

    pub fn new(tag: &str) -> Node {
        Node {
            tag: tag.to_string(),
            children: vec![]
        }
    }

    pub fn append_to(self: Rc<Self>, parent: &mut Node) {
        parent.children.push(self);
    }

}

#[test]
fn test_node () {
    let mut top = Node::new("Top");
    let top1 = Rc::new(Node::new("Top_1"));
    let top2 = Rc::new(Node::new("Top_2"));
    top1.append_to(&mut top);
    top2.append_to(&mut top);

    assert_eq!(top.children[0].tag, "Top_1");
    assert_eq!(top.children[1].tag, "Top_2");
}