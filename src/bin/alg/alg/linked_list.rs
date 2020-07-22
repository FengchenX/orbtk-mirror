
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

type Opt<T> = Option<Box<T>>;
#[derive(Clone)]
struct ListNode {
    val: i32,
    next: Opt<ListNode>
}

fn merge_two_lists(l1: Opt<ListNode>, l2: Opt<ListNode>) -> Opt<ListNode> {
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
    let mut l13 = Some(Box::new(ListNode{val: 4, next: None}));
    let mut l12 = Some(Box::new(ListNode{val: 2, next: l13}));
    let mut l11 = Some(Box::new(ListNode{val: 1, next: l12}));
    let mut l23 = Some(Box::new(ListNode{val: 4, next: None}));
    let mut l22 = Some(Box::new(ListNode{val: 3, next: l23}));
    let mut l21 = Some(Box::new(ListNode{val: 1, next: l22}));

    let mut res = merge_two_lists(l11, l21);
    while let Some(ref re) = res {
       println!("{}", re.val);
        res = re.next.clone();
    }
}

/*
24. 两两交换链表中的节点
给定一个链表，两两交换其中相邻的节点，并返回交换后的链表。
你不能只是单纯的改变节点内部的值，而是需要实际的进行节点交换。
示例:
给定 1->2->3->4, 你应该返回 2->1->4->3.
1   ->  2  ->  1  -> None
cur    next
       prev

3    ->  4     ->3  -> None
head
cur      next
         prev
*/
fn swap_pairs(head: Opt<ListNode>) ->Opt<ListNode> {
    let mut dummy_head = ListNode{val:0, next:None};
    let mut prev = &mut dummy_head.next;
    let mut head = head;
    while let Some(mut cur) = head.take() {
        if let Some(mut next) = cur.next.take() {
            head = next.next.replace(cur);
            prev.replace(next);
            prev = &mut prev.as_mut().unwrap().next.as_mut().unwrap().next;
        } else {
            prev.replace(cur);
        }
    }
    dummy_head.next
}

pub fn test2() {
    let l4 = Some(Box::new(ListNode { val: 4, next: None }));
    let l3 = Some(Box::new(ListNode { val: 3, next: l4 }));
    let l2 = Some(Box::new(ListNode { val: 2, next: l3 }));
    let mut head = Some(Box::new(ListNode { val: 1, next: l2 }));
    let u = swap_pairs(head);
    if let Some(p) = u {
        println!("{},{},{}", p.val, p.next.as_ref().unwrap().val, p.next.as_ref().unwrap().next.as_ref().unwrap().val);
    }
}