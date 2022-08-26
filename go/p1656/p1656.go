package p1656

type OrderedStream struct {
	stream []string
	ptr    int
}

func Constructor(n int) OrderedStream {
	return OrderedStream{make([]string, n), 0}
}

func (this *OrderedStream) Insert(idKey int, value string) []string {
	res := make([]string, 0)
	this.stream[idKey-1] = value
	i := this.ptr
	for i < len(this.stream) {
		if this.stream[i] == "" {
			break
		}
		res = append(res, this.stream[i])
		i++
	}
	this.ptr = i
	return res
}
