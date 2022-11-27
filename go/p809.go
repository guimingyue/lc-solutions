package leetcode

func expressiveWords(s string, words []string) int {
	chs := make([]rune, 0)
	nums := make([]int, 0)
	for _, ch := range s {
		if len(chs) > 0 && chs[len(chs)-1] == ch {
			nums[len(chs)-1]++
		} else {
			chs = append(chs, ch)
			nums = append(nums, 1)
		}
	}

	res := 0
	for _, word := range words {
		if expressive(word, chs, nums) {
			res++
		}
	}
	return res
}

func expressive(word string, chs []rune, nums []int) bool {
	ch := word[0]
	num := 1
	idx := 0
	for i := 1; i <= len(word); i++ {
		if i == len(word) || ch != word[i] {
			if idx >= len(chs) || chs[idx] != rune(ch) {
				return false
			}
			if (num != nums[idx] && nums[idx] < 3) || num > nums[idx] {
				return false
			}
			idx++
			if i == len(word) {
				break
			}
			ch = word[i]
			num = 1
		} else {
			num++
		}
	}
	if idx < len(chs) {
		return false
	}
	return true
}
