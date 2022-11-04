package leetcode

func countMatches(items [][]string, ruleKey string, ruleValue string) int {
	res := 0
	for _, item := range items {
		if ruleKey == "type" && ruleValue == item[0] {
			res++
		} else if ruleKey == "color" && ruleValue == item[1] {
			res++
		} else if ruleKey == "name" && ruleValue == item[2] {
			res++
		}
	}
	return res
}
