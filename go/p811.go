package leetcode

import (
	"github.com/emirpasic/gods/utils"
	"strconv"
	"strings"
)

func subdomainVisits(cpdomains []string) []string {
	m := make(map[string]int)
	for _, s := range cpdomains {
		sdomains := strings.Split(s, " ")
		num, _ := strconv.Atoi(sdomains[0])
		strs := strings.Split(sdomains[1], ".")
		l := len(strs)
		var subDomain string
		for i := l - 1; i >= 0; i-- {
			if i < l-1 {
				subDomain = strs[i] + "." + subDomain
			} else {
				subDomain = strs[i]
			}

			m[subDomain] += num
		}
	}
	var res []string
	for k, v := range m {
		res = append(res, utils.ToString(v)+" "+k)
	}
	return res
}
