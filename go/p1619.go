package leetcode

import (
	"github.com/emirpasic/gods/trees/binaryheap"
	"github.com/emirpasic/gods/utils"
)

func trimMean(arr []int) float64 {
	l := len(arr)
	n := l / 20
	// 小顶堆存较大的数
	heapMin := binaryheap.NewWithIntComparator()
	// 大顶堆存较小的树
	heapMax := binaryheap.NewWith(func(a, b interface{}) int {
		return utils.IntComparator(b, a)
	})
	sum := 0

	for _, v := range arr {
		// 较小的数
		if heapMax.Size() < n {
			heapMax.Push(v)
		} else {
			v1, _ := heapMax.Peek()
			if v1.(int) > v {
				heapMax.Pop()
				heapMax.Push(v)
			}
		}

		// 较大的数
		if heapMin.Size() < n {
			heapMin.Push(v)
		} else {
			v1, _ := heapMin.Peek()
			if v1.(int) < v {
				heapMin.Pop()
				heapMin.Push(v)
			}
		}
		sum += v
	}
	it := heapMax.Iterator()
	for it.Next() {
		sum -= it.Value().(int)
	}
	it = heapMin.Iterator()
	for it.Next() {
		sum -= it.Value().(int)
	}
	return float64(sum) / float64(l-n-n)
}
