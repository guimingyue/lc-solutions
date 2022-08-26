package leetcode

type MovingAverage struct {
	quque []int
	size  int
	sum   int
}

/** Initialize your data structure here. */
func Constructor(size int) MovingAverage {
	return MovingAverage{
		quque: make([]int, 0),
		size:  size,
		sum:   0,
	}
}

func (this *MovingAverage) Next(val int) float64 {
	if len(this.quque) < this.size {
		this.quque = append(this.quque, val)
	} else {
		t := this.quque[0]
		this.quque = this.quque[1:]
		this.quque = append(this.quque, val)
		this.sum -= t
	}
	this.sum += val
	return float64(this.sum) / float64(len(this.quque))
}
