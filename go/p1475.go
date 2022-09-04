package leetcode

func finalPrices(prices []int) []int {
	res := make([]int, len(prices))
	for i := 0; i < len(prices); i++ {
		j := i + 1
		for ; j < len(prices); j++ {
			if prices[j] <= prices[i] {
				break
			}
		}
		if j < len(prices) {
			res[i] = prices[i] - prices[j]
		} else {
			res[i] = prices[i]
		}
	}
	return res
}
