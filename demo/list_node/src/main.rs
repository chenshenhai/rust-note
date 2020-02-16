
// 链表节点
#[derive(PartialEq, Eq, Clone, Debug)]
struct Node {
    pub value: i32,
    pub next: Option<Box<Node>>
}

// 链表
impl Node {
    #[inline]
    fn new(value: i32) -> Self {
        Node {
            value: value,
            next: None
        }
    }
}

// 创建链表
fn create_list(start: i32, count: i32, unit: i32) -> Option<Box<Node>> {
    let mut head_node = Some(Box::new(Node::new(start)));
    let mut prev_node = &mut head_node;
    for i in 0..count as usize {
        match prev_node {
            Some(node) => {
                let idx = (i + 1) as i32;
                node.next = Some(Box::new(Node::new(idx * unit)));
                prev_node = &mut node.next;
            },
            None => {},
        }
    } 
    head_node
}

fn main() {
    let list = create_list(1, 4, 1);
    println!("{:?}", list);
}
