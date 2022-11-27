package leetcode

func check(nums []int) bool {
	var i int
	for i = 1; i < len(nums); i++ {
		if nums[i] < nums[i-1] {
			break
		}
	}
	if i < len(nums) {
		if nums[i] > nums[0] {
			return false
		}
		for ; i < len(nums)-1; i++ {
			if nums[i] > nums[i+1] || nums[i] > nums[0] || nums[i+1] > nums[0] {
				return false
			}
		}
	}
	return true
}

/*
[1,3,2]
[3,1,2,2,4]
[2,1,3,4]
[3,4,5,1,2]
[1,2,3]
[1,2]
[1]
*/
