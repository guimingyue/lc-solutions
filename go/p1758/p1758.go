package p1758

func minOperations(s string) int {
	next := func(ch uint8) uint8 {
		if ch == '0' {
			return '1'
		} else {
			return '0'
		}
	}
	operations := func(cur uint8, s string) (count int) {
		if cur == s[0] {
			count = 0
		} else {
			count = 1
		}
		i := 1
		for ; i < len(s); i++ {
			ch := s[i]
			if cur != ch {
				cur = ch
			} else {
				cur = next(ch)
				count++
			}
		}
		return
	}
	c1 := operations('0', s)
	if c1 == 0 {
		return c1
	}
	c2 := operations('1', s)
	if c1 < c2 {
		return c1
	}
	return c2
}
