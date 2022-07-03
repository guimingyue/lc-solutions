func findRepeatNumber(nums []int) int {
    for i := 0; i < len(nums); i++ {
		for i != nums[i] {
			if nums[nums[i]] == nums[i] {
				return nums[i]
			}
			nums[i], nums[nums[i]] = nums[nums[i]], nums[i]
		}
	}
	return -1
}   