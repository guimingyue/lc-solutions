package p895

type FreqStack struct {
	freq map[int]int

	group map[int][]int

	maxFreq int
}

func Constructor() FreqStack {
	return FreqStack{
		freq:    make(map[int]int),
		group:   make(map[int][]int),
		maxFreq: 0,
	}
}

func (this *FreqStack) Push(val int) {
	this.freq[val]++
	this.group[this.freq[val]] = append(this.group[this.freq[val]], val)
	if this.freq[val] > this.maxFreq {
		this.maxFreq = this.freq[val]
	}
}

func (this *FreqStack) Pop() int {
	popGroup := this.group[this.maxFreq]
	l := len(popGroup)
	val := popGroup[l-1]
	this.group[this.maxFreq] = popGroup[:l-1]
	this.freq[val]--
	if l-1 == 0 {
		this.maxFreq = this.freq[val]
	}
	return val
}
