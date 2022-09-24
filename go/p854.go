package leetcode

func kSimilarity(s1 string, s2 string) int {
	type Pair struct {
		s string
		i int
	}
	q := []Pair{{s1, 0}}
	vis := map[string]bool{s1: true}
	step := 0
	for n := len(s1); ; step++ {
		tmp := q
		q = nil
		for _, p := range tmp {
			s, i := p.s, p.i
			if s == s2 {
				return step
			}
			for i < n && s[i] == s2[i] {
				i++
			}
			t := []byte(s)
			for j := i + 1; j < n; j++ {
				if s[j] == s2[i] && s[j] != s2[j] {
					t[i], t[j] = t[j], t[i]
					if st := string(t); !vis[st] {
						vis[st] = true
						q = append(q, Pair{st, i})
					}
					t[i], t[j] = t[j], t[i]
				}
			}
		}

	}
}

/*
"eacd"
"acde"


*/
