package leetcode

import (
	"math"
	"sort"
)

func minimumAbsDifference(arr []int) [][]int {
	sort.Ints(arr)
	min := math.MaxInt32
	res := make([][]int, 0)
	for i := 0; i < len(arr)-1; i++ {
		dlt := arr[i+1] - arr[i]
		if dlt <= min {
			if dlt < min {
				min = dlt
				res = nil
			}
			res = append(res, []int{arr[i], arr[i+1]})
		}
	}
	return res
}
