package leetcode

import "unicode"

func secondHighest(s string) int {
	max, sec := -1, -1
	for _, v := range s {
		if unicode.IsDigit(v) {
			num := int(v - '0')
			if num == max {
				continue
			} else if num > max {
				sec = max
				max = num
			} else if num < max && num > sec {
				sec = num
			}
		}
	}
	return sec
}
