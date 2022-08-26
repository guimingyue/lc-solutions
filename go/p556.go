package leetcode

import (
	"math"
	"sort"
)

func nextGreaterElement(n int) int {
	arr := make([]int, 0)
	for n > 0 {
		arr = append(arr, n%10)
		n = n / 10
	}
	i := 1
	for ; i < len(arr); i++ {
		if arr[i-1] > arr[i] {
			break
		}
	}
	if i == len(arr) {
		return -1
	}
	j := 0
	for ; j < i; j++ {
		if arr[j] > arr[i] {
			break
		}
	}

	temp := arr[i]
	arr[i] = arr[j]
	arr[j] = temp

	sort.Slice(arr[0:i], func(i, j int) bool {
		return arr[j]-arr[i] < 0
	})

	res := int64(0)
	for idx := len(arr) - 1; idx >= 0; idx-- {
		res = res*10 + int64(arr[idx])
	}
	if res > math.MaxInt32 {
		return -1
	}
	return int(res)
}
