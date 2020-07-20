
/*
链表题解题模版
说明：日常生活中双链表是最常见的，毕竟好操作嘛，但是吧，面试的时候往往是单链表，可能更考验出人的能力？
func LinkedList(root){
  pre := nil //  维护一个前节点，极其重要，望周知。
  cur := root
  for cur != nil {
    if pre == nil {
       ////
    }else {
       /////
    }
    pre = cur
    cur = cur.next
  }
}
*/

/*
21. 合并两个有序链表
将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。
示例：
输入：1->2->4, 1->3->4
输出：1->1->2->3->4->4
*/

type Node<T> = Option<Box<T>>;
#[derive(Clone)]
struct List {
    val: i32,
    next: Node<List>
}

fn merge_two_lists(l1: Node<List>, l2: Node<List>) -> Node<List> {
    match (l1, l2) {
        (None, None) => None,
        (None, r) => r,
        (l, None) => l,
        (Some(mut l), Some(mut r)) => {
            if l.val <= r.val {
                l.next = merge_two_lists(l.next, Some(r));
                Some(l)
            } else {
                r.next = merge_two_lists(Some(l), r.next);
                Some(r)
            }
        }
    }
}

pub fn test() {
    let mut l13 = Some(Box::new(List{val: 4, next: None}));
    let mut l12 = Some(Box::new(List{val: 2, next: l13}));
    let mut l11 = Some(Box::new(List{val: 1, next: l12}));
    let mut l23 = Some(Box::new(List{val: 4, next: None}));
    let mut l22 = Some(Box::new(List{val: 3, next: l23}));
    let mut l21 = Some(Box::new(List{val: 1, next: l22}));

    let mut res = merge_two_lists(l11, l21);
    while let Some(ref re) = res {
       println!("{}", re.val);
        res = re.next.clone();
    }
}
