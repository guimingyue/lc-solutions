import (
	"github.com/emirpasic/gods/sets/hashset"
	"math/rand"
)

type Solution struct {
	b2w   map[int]int
	bound int
}

func Constructor(n int, blacklist []int) Solution {
	set := hashset.New()
	bound := n - len(blacklist)
	for _, v := range blacklist {
		if v >= bound {
			set.Add(v)
		}
	}
	w := bound
	b2w := make(map[int]int, 0)
	for _, v := range blacklist {
		if v >= bound {
			continue
		}
		for set.Contains(w) {
			w++
		}
		b2w[v] = w
		w++
	}

	return Solution{
		b2w:   b2w,
		bound: bound,
	}
}

func (this *Solution) Pick() int {
	k := rand.Intn(this.bound)
	v, ok := this.b2w[k]
	if ok {
		return v
	} else {
		return k
	}
}