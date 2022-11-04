package leetcode

import "strings"

func magicalString(n int) int {
	total := 3
	res := 1
	s := "2"
	lastCh := '2'
	for total < n {
		apps := ""
		for _, ch := range s {
			cnt := int(ch - '0')
			if total+cnt > n {
				cnt = n - total
			}
			total += cnt
			var tmp string
			if lastCh == '1' {
				tmp = strings.Repeat("2", cnt)
				lastCh = '2'
			} else {
				tmp = strings.Repeat("1", cnt)
				lastCh = '1'
				res += cnt
			}
			apps += tmp
			if total >= n {
				return res
			}
		}
		s = apps
	}
	return res
}
