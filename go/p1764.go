package leetcode

func canChoose(groups [][]int, nums []int) bool {
	i := 0
	for _, group := range groups {
		kg := 0
		j := i
		for kg < len(group) {
			if i >= len(nums) {
				return false
			}
			if nums[i] == group[kg] {
				kg++
				i++
			} else {
				kg = 0
				j++
				i = j

			}
		}
	}
	return true
}
