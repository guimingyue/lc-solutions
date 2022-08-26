package p676

import (
	"github.com/emirpasic/gods/sets/hashset"
)

type MagicDictionary struct {
	set *hashset.Set
}

func Constructor() MagicDictionary {
	return MagicDictionary{
		set: hashset.New(),
	}
}

func (this *MagicDictionary) BuildDict(dictionary []string) {
	for _, s := range dictionary {
		this.set.Add(s)
	}
}

func (this *MagicDictionary) Search(searchWord string) bool {
	for i, ch := range searchWord {
		for c := 'a'; c <= 'z'; c++ {
			if ch == c {
				continue
			}

			w := searchWord[0:i] + string(c) + searchWord[i+1:]
			if this.set.Contains(w) {
				return true
			}
		}
	}
	return false
}
