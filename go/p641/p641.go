package p641

type MyCircularDeque struct {
	front, rear int
	elements    []int
}

func Constructor(k int) MyCircularDeque {
	return MyCircularDeque{elements: make([]int, k+1)}
}

func (this *MyCircularDeque) InsertFront(value int) bool {
	if this.IsFull() {
		return false
	}
	this.front = (this.front - 1 + len(this.elements)) % len(this.elements)
	this.elements[this.front] = value
	return true
}

func (this *MyCircularDeque) InsertLast(value int) bool {
	if this.IsFull() {
		return false
	}
	this.elements[this.rear] = value
	this.rear = (this.rear + 1 + len(this.elements)) % len(this.elements)
	return true
}

func (this *MyCircularDeque) DeleteFront() bool {
	if this.IsEmpty() {
		return false
	}
	this.front = (this.front + 1 + len(this.elements)) % len(this.elements)
	return true
}

func (this *MyCircularDeque) DeleteLast() bool {
	if this.IsEmpty() {
		return false
	}
	this.rear = (this.rear - 1 + len(this.elements)) % len(this.elements)
	return true
}

func (this *MyCircularDeque) GetFront() int {
	if this.IsEmpty() {
		return -1
	}
	return this.elements[this.front]
}

func (this *MyCircularDeque) GetRear() int {
	if this.IsEmpty() {
		return -1
	}
	idx := (this.rear - 1 + len(this.elements)) % len(this.elements)
	return this.elements[idx]
}

func (this *MyCircularDeque) IsEmpty() bool {
	return this.front == this.rear
}

func (this *MyCircularDeque) IsFull() bool {
	return (this.rear+1)%len(this.elements) == this.front
}
