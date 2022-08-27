package leetcode

import (
	"sort"
)

func findClosestElements(arr []int, k int, x int) []int {
	idx := sort.SearchInts(arr, x)
	l := idx
	r := idx
	c := 0
	if idx < len(arr) {
		if arr[idx] == x {
			c += 1
			l--
			r++
		} else {
			l = idx - 1
		}
	} else {
		l = len(arr) - k - 1
		r = len(arr)
		c = k
	}
	abs := func(a, b int) int {
		v := a - b
		if v < 0 {
			return -v
		}
		return v
	}
	for l >= 0 && r < len(arr) {
		if abs(arr[l], x) <= abs(arr[r], x) {
			l--
		} else {
			r++
		}
		c++
		if c == k {
			break
		}
	}
	if c < k {
		for l >= 0 && c < k {
			l--
			c++
		}
		for r < len(arr) && c < k {
			r++
			c++
		}
	}
	return arr[l+1 : r]
}
