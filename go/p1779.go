package leetcode

import "math"

func nearestValidPoint(x int, y int, points [][]int) int {
	abs := func(v int) int {
		if v < 0 {
			return -v
		}
		return v
	}
	maxDis := math.MaxInt32
	res := -1
	for i, p := range points {
		if p[0] != x && p[1] != y {
			continue
		}
		dis := abs(p[0]-x) + abs(p[1]-y)
		if dis < maxDis {
			res = i
			maxDis = dis
		}
	}
	return res
}
