package leetcode

import (
	"sort"
	"strings"
)

func findLUSlength(strs []string) int {
	sort.Slice(strs, func(i, j int) bool {
		l1 := len(strs[i])
		l2 := len(strs[j])
		if l1 == l2 {
			return strings.Compare(strs[i], strs[j]) < 0
		} else {
			return l1 < l2
		}
	})

	isSubsequence := func(s1 string, s2 string) bool {
		i, j := 0, 0
		for i < len(s1) && j < len(s2) {
			if s1[i] == s2[j] {
				i++
				j++
			} else {
				j++
			}
		}
		return i == len(s1)
	}

	max := -1
	for idx, str := range strs {
		isSub := false
		if idx > 0 && str == strs[idx-1] {
			continue
		}
		for i := idx + 1; i < len(strs); i++ {
			if isSubsequence(str, strs[i]) {
				isSub = true
				break
			}
		}
		if !isSub {
			if max < len(str) {
				max = len(str)
			}
		}
	}
	return max
}
