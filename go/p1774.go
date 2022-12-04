package leetcode

import "math"

func closestCost(baseCosts []int, toppingCosts []int, target int) int {
	res := math.MaxInt32
	curAbs := abs(res - target)
	var costDfs func(tcs []int, idx, target, cur, curRes int)
	costDfs = func(tcs []int, idx, target, cur, curRes int) {
		if idx == len(tcs) {
			a := abs(cur - target)
			if a < curAbs {
				res = cur
				curAbs = a
			} else if a == curAbs {
				res = minVal(cur, res)
			}
			return
		}
		if cur > target && abs(cur-target) > abs(curRes-target) {
			return
		}
		costDfs(tcs, idx+1, target, cur, curRes)
		costDfs(tcs, idx+1, target, cur+tcs[idx], curRes)
		costDfs(tcs, idx+1, target, cur+tcs[idx]+tcs[idx], curRes)
	}
	for _, c := range baseCosts {
		costDfs(toppingCosts, 0, target, c, res)
	}
	return res
}

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func minVal(a, b int) int {
	if a < b {
		return a
	}
	return b
}
