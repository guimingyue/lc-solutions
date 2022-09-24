package p707

type MyLinkedList struct {
	tail *Node

	head *Node

	len int
}

type Node struct {
	val int

	prev *Node

	next *Node
}

func Constructor() MyLinkedList {
	return MyLinkedList{
		len:  0,
		tail: nil,
		head: nil,
	}
}

func (list *MyLinkedList) Get(index int) int {
	p := list.GetNode(index)
	if p == nil {
		return -1
	}
	return p.val
}

func (list *MyLinkedList) GetNode(index int) *Node {
	if list.len <= index {
		return nil
	}
	var p *Node
	if index <= list.len/2 {
		p = list.head
		for index > 0 {
			index--
			p = p.next
		}
	} else {
		end := list.len - 1
		p = list.tail
		for end > index {
			p = p.prev
			end--
		}
	}
	return p
}

func (list *MyLinkedList) AddAtHead(val int) {
	if list.head == nil {
		list.head = &Node{val: val, next: nil, prev: nil}
		list.tail = list.head
	} else {
		next := list.head
		list.head = &Node{val: val, next: next, prev: nil}
		next.prev = list.head
	}
	list.len++
}

func (list *MyLinkedList) AddAtTail(val int) {
	if list.head == nil {
		list.head = &Node{val: val, next: nil, prev: nil}
		list.tail = list.head
	} else {
		prev := list.tail
		list.tail = &Node{val: val, prev: prev, next: nil}
		prev.next = list.tail
	}
	list.len++
}

func (list *MyLinkedList) AddAtIndex(index int, val int) {
	if list.len < index {
		return
	} else if list.len == index {
		list.AddAtTail(val)
	} else if index <= 0 {
		list.AddAtHead(val)
	} else {
		p := list.GetNode(index)
		n := &Node{val: val, prev: p.prev, next: p}
		if p.prev != nil {
			p.prev.next = n
		}
		p.prev = n
		list.len++
	}

}

func (list *MyLinkedList) DeleteAtIndex(index int) {
	if list.len <= index {
		return
	}
	if list.len == 1 {
		list.head = nil
		list.tail = list.head
	} else {
		p := list.GetNode(index)
		if p == list.head {
			list.head = p.next
		}
		if p == list.tail {
			list.tail = p.prev
		}
		if p.prev != nil {
			p.prev.next = p.next
		}
		if p.next != nil {
			p.next.prev = p.prev
		}

	}
	list.len--
}

/**
["MyLinkedList","addAtHead","addAtHead","addAtHead","addAtIndex","deleteAtIndex","addAtHead","addAtTail","get","addAtHead","addAtIndex","addAtHead"]
[[],[7],[2],[1],[3,0],[2],[6],[4],[4],[4],[5,0],[6]]

["MyLinkedList","addAtHead","deleteAtIndex"]
[[],[1],[0]]

["MyLinkedList","addAtHead","addAtTail","addAtHead","addAtTail","addAtHead","addAtHead","get","addAtHead","get","get","addAtTail"]
[[],[7],[7],[9],[8],[6],[0],[5],[0],[2],[5],[4]]

["MyLinkedList","addAtHead","addAtTail","addAtIndex","get","deleteAtIndex","get"]
[[],[1],[3],[1,2],[1],[0],[0]]

["MyLinkedList","addAtIndex","addAtIndex","addAtIndex","get"]
[[],[0,10],[0,20],[1,30],[0]]

["MyLinkedList","addAtHead","get","addAtHead","addAtHead","deleteAtIndex","addAtHead","get","get","get","addAtHead","deleteAtIndex"]
[[],[4],[1],[1],[5],[3],[7],[3],[3],[3],[1],[4]]

["MyLinkedList","addAtHead","addAtTail","addAtIndex"]
[[],[1],[3],[3,2]]
*/
