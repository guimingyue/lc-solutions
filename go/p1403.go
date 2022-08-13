import "sort"

func minSubsequence(nums []int) []int {
	sum := 0
	for _, v := range nums {
		sum += v
	}
	res := make([]int, 0)
	sort.Ints(nums)
	s := 0
	for i := len(nums) - 1; i >= 0; i-- {
		res = append(res, nums[i])
		s += nums[i]
		sum -= nums[i]
		if s > sum {
			break
		}
	}
	return res
}
