package leetcode

import "strings"

func letterCasePermutation(s string) []string {
	const delta = 'a' - 'A'
	var dfs func(s string, m map[string]struct{}, idx int)
	dfs = func(s string, m map[string]struct{}, idx int) {
		if idx == len(s) {
			return
		}
		if s[idx] >= '0' && s[idx] <= '9' {
			dfs(s, m, idx+1)
		} else {
			dfs(s, m, idx+1)
			var ss string
			if s[idx] >= 'a' && s[idx] <= 'z' {
				ss = s[0:idx] + strings.ToUpper(string(s[idx])) + s[idx+1:]
			} else {
				ss = s[0:idx] + strings.ToLower(string(s[idx])) + s[idx+1:]
			}
			m[ss] = struct{}{}
			dfs(ss, m, idx+1)
		}
	}
	set := map[string]struct{}{}
	set[s] = struct{}{}
	dfs(s, set, 0)
	var res []string
	for k := range set {
		res = append(res, k)
	}

	return res
}
