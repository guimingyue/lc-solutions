package leetcode

import (
	"strings"
)

func reformatNumber(number string) string {
	numCnt := 0
	builder := strings.Builder{}
	for _, ch := range number {
		if ch >= '0' && ch <= '9' {
			numCnt++
			builder.WriteRune(ch)
			if numCnt%3 == 0 {
				builder.WriteRune('-')
			}
		}
	}
	res := builder.String()
	if numCnt%3 == 0 {
		return res[0 : len(res)-1]
	} else if numCnt%3 == 1 {
		l := len(res)
		return res[0:l-3] + res[l-2:l-1] + res[l-3:l-2] + res[l-1:]
	} else {
		return res
	}
}
