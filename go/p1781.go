package leetcode

func beautySum(s string) int {
	res := 0
	for i := 0; i <= len(s)-2; i++ {
		m := make(map[uint8]int)
		num := make(map[int]int)
		m[s[i]]++
		m[s[i+1]]++
		var max, min int
		if s[i] == s[i+1] {
			max = 2
			min = 0
			num[max] = 1
		} else {
			max = 1
			min = 1
			num[max] = 2
		}
		for j := i + 2; j < len(s); j++ {
			cur := m[s[j]]
			m[s[j]]++
			if cur == 0 {
				min = 1
				num[min]++
			} else {
				num[cur]--
				num[cur+1]++
				if cur+1 > max {
					max = cur + 1
				}
				if cur == min && num[cur] == 0 {
					min = cur + 1
				}
			}
			if min != 0 {
				res += max - min
			}
		}
	}
	return res
}

/**
"baa"
"cbaa"
"bcb"
"aabcb"
"aabcbaa"
**/
