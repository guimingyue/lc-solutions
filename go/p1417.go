package leetcode

import "strings"

func reformat(s string) string {
	nums := make([]rune, 0)
	chs := make([]rune, 0)
	for _, v := range s {
		if v >= '0' && v <= '9' {
			nums = append(nums, v)
		} else {
			chs = append(chs, v)
		}
	}
	var l []rune
	var r []rune
	if len(nums) >= len(chs) && len(nums)-len(chs) <= 1 {
		l = nums
		r = chs
	} else if len(nums) <= len(chs) && len(chs)-len(nums) <= 1 {
		l = chs
		r = nums
	} else {
		return ""
	}
	builder := strings.Builder{}
	i := 0
	for ; i < len(r); i++ {
		builder.WriteByte(byte(l[i]))
		builder.WriteByte(byte(r[i]))
	}
	if i < len(l) {
		builder.WriteByte(byte(l[i]))
	}
	return builder.String()
}
