package leetcode

import (
	"sort"
	"strings"
)

func customSortString(order string, s string) string {
	m := make(map[byte]int)
	for i, ch := range order {
		m[byte(ch)] = i
	}
	res := make([]byte, len(s))
	for i, ch := range s {
		res[i] = byte(ch)
	}

	sort.Slice(res, func(i, j int) bool {
		var ci, cj int
		if c, ok := m[res[i]]; ok {
			ci = c
		} else {
			ci = len(res)
		}

		if c, ok := m[res[j]]; ok {
			cj = c
		} else {
			cj = len(res)
		}
		return ci < cj
	})
	buider := strings.Builder{}
	buider.Write(res)
	return buider.String()
}
