package leetcode

func countNicePairs(nums []int) int {
	const MOD = 1_000_000_007
	n := len(nums)
	revNums := make([]int, n)
	revFunc := func(num int) (res int) {
		res = 0
		for num > 0 {
			res = res*10 + num%10
			num = num / 10
		}
		return res
	}

	m := make(map[int]int)
	res := 0
	for i := 0; i < n; i++ {
		revNums[i] = revFunc(nums[i])
		diff := nums[i] - revNums[i]
		if num, ok := m[diff]; ok {
			res = (res + num) % MOD
		}
		m[diff] += 1
	}
	return res
}
