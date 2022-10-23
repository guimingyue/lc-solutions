package p901

import "math"

type StockSpanner struct {
	stack [][2]int
	idx   int
}

func Constructor() StockSpanner {
	return StockSpanner{
		stack: [][2]int{{-1, math.MaxInt32}},
		idx:   -1,
	}
}

func (this *StockSpanner) Next(price int) int {
	this.idx++
	for price >= this.stack[len(this.stack)-1][1] {
		this.stack = this.stack[0 : len(this.stack)-1]
	}
	ret := this.idx - this.stack[len(this.stack)-1][0]
	this.stack = append(this.stack, [2]int{this.idx, price})
	return ret
}
