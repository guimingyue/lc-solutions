package leetcode

func ambiguousCoordinates(s string) []string {
	var ambiguous func(s string) []string
	ambiguous = func(s string) []string {
		var res []string
		for i := 1; i < len(s); i++ {
			left := enum(s[0:i])
			right := enum(s[i:])
			for _, vl := range left {
				for _, vr := range right {
					res = append(res, "("+vl+", "+vr+")")
				}
			}
		}
		return res
	}
	return ambiguous(s[1 : len(s)-1])
}

func enum(s string) []string {
	if len(s) == 1 {
		return []string{s}
	}
	if s[0] == '0' && s[len(s)-1] == '0' {
		return []string{}
	}
	if s[0] != '0' && s[len(s)-1] == '0' {
		return []string{s}
	}
	if s[0] == '0' {
		return []string{"0." + s[1:]}
	}
	var res []string
	for i := 1; i <= len(s); i++ {
		sv := s[0:i]
		if len(s[i:]) > 0 {
			sv += "."
		}
		res = append(res, sv+s[i:])
	}
	return res
}
