type MyCircularQueue struct {
	back []int
	h, t int
	full bool
}

func Constructor(k int) MyCircularQueue {
	arr := make([]int, k)
	return MyCircularQueue{back: arr}
}

func (this *MyCircularQueue) EnQueue(value int) bool {
	if this.IsFull() {
		return false
	}
	this.back[this.h] = value
	this.h = (this.h + 1 + len(this.back)) % len(this.back)
	if this.h == this.t {
		this.full = true
	}
	return true
}

func (this *MyCircularQueue) DeQueue() bool {
	if this.IsEmpty() {
		return false
	}
	this.t = (this.t + 1 + len(this.back)) % len(this.back)
	this.full = false
	return true
}

func (this *MyCircularQueue) Front() int {
	if this.IsEmpty() {
		return -1
	}
	return this.back[this.t]
}

func (this *MyCircularQueue) Rear() int {
	if this.IsEmpty() {
		return -1
	}
	return this.back[(this.h-1+len(this.back))%len(this.back)]
}

func (this *MyCircularQueue) IsEmpty() bool {
	if !this.full && this.h == this.t {
		return true
	}
	return false
}

func (this *MyCircularQueue) IsFull() bool {
	return this.full
}