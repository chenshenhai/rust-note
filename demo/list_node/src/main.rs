
// 链表节点
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub value: i32,
    pub next: Option<Box<ListNode>>
}

// 链表
impl ListNode {
    #[inline]
    fn new(value: i32) -> Self {
        ListNode {
            value: value,
            next: None
        }
    }
}

// 创建链表
fn create_list(start: i32, count: i32, unit: i32) -> Option<Box<ListNode>> {
    let mut head_node = Some(Box::new(ListNode::new(start)));
    let mut prev_node = &mut head_node;
    for i in 1..count as usize {
        match prev_node {
            Some(node) => {
                let idx = (i + 1) as i32;
                node.next = Some(Box::new(ListNode::new(idx * unit)));
                prev_node = &mut node.next;
            },
            None => {},
        }
    } 
    head_node
}

// 遍历读取链表节点
fn read_list (list: Option<Box<ListNode>>) {
    let mut list = list;
    let mut node = &mut list;
    while node.is_some() {
        println!("{}", node.as_ref().unwrap().value);

        if node.is_some() {
            node = &mut node.as_mut().unwrap().next;
        } else {
            break;
        }
    }
}

fn main() {
    let list = create_list(1, 4, 1);
    println!("println list format: ");
    println!("{:?}", list);

    println!("println all nodes of list: ");
    read_list(list);
}
