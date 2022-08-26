package p745

type WordFilter struct {
	m map[string]int
}

func Constructor(words []string) WordFilter {
	m := make(map[string]int)
	for i, word := range words {
		for plen := 1; plen <= len(word); plen++ {
			for slen := 1; slen <= len(word); slen++ {
				m[word[0:plen]+"#"+word[len(word)-slen:]] = i
			}
		}
	}
	return WordFilter{m}
}

func (this *WordFilter) F(pref string, suff string) int {
	idx, o := this.m[pref+"#"+suff]
	if o {
		return idx
	}
	return -1
}
