package leetcode

func sortPeople(names []string, heights []int) []string {
    idx := make([]int, len(heights))
	for i := 0; i < len(idx); i++ {
		idx[i] = i
	}
	sort.Slice(idx, func(i, j int) bool {
		return heights[idx[i]] > heights[idx[j]]
	})
	res := make([]string, len(names))
	for i := 0; i < len(res); i++ {
		res[i] = names[idx[i]]
	}
	return res
}