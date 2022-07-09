import (
	"github.com/emirpasic/gods/trees/redblacktree"
	"math"
)

type MyCalendar struct {
	t *redblacktree.Tree
}

func Constructor() MyCalendar {
	t := redblacktree.NewWithIntComparator()
	t.Put(math.MaxInt32, nil)
	return MyCalendar{t}
}

func (this *MyCalendar) Book(start int, end int) bool {
	node, _ := this.t.Ceiling(end)
	it := this.t.IteratorAt(node)
	if !it.Prev() || it.Value().(int) <= start {
		this.t.Put(start, end)
		return true
	}
	return false
}