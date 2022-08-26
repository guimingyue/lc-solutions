package leetcode

import (
	"bytes"
	"strconv"
)

func diffWaysToCompute(expression string) []int {
	res := make([]int, 0)
	if len(expression) == 0 {
		return res
	}
	ops := []byte{'+', '-', '*'}
	containOp := false
	for idx, ch := range expression {
		if bytes.IndexByte(ops, byte(ch)) < 0 {
			continue
		}
		containOp = true
		left := diffWaysToCompute(expression[0:idx])
		right := diffWaysToCompute(expression[idx+1:])
		for _, vl := range left {
			for _, vr := range right {
				if ch == '+' {
					res = append(res, vl+vr)
				} else if ch == '-' {
					res = append(res, vl-vr)
				} else {
					res = append(res, vl*vr)
				}
			}
		}
	}
	if !containOp {
		v, _ := strconv.Atoi(expression)
		res = append(res, v)
	}
	return res
}
