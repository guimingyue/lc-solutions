package leetcode

func decrypt(code []int, k int) []int {
	dirt := 1
	num := k
	if k < 0 {
		num = -k
		dirt = -1
	} else if k == 0 {
		num = 0
		dirt = 0
	}
	res := make([]int, len(code))
	for i, _ := range code {
		n := 0
		rv := 0
		j := i
		for n < num {
			n++
			j = (j + dirt + len(code)) % len(code)
			rv += code[j]
		}
		res[i] = rv
	}
	return res
}
