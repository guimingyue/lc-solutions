package leetcode

func isPrefixOfWord(sentence string, searchWord string) int {
	i := 0
	idx := 0
	for i < len(sentence) {
		if sentence[i] == ' ' {
			i++
			continue
		}
		j := 0
		for i < len(sentence) && j < len(searchWord) {
			if sentence[i] != searchWord[j] {
				break
			}
			i++
			j++
		}
		if j == len(searchWord) {
			return idx + 1
		}
		for i < len(sentence) && sentence[i] != ' ' {
			i++
		}
		idx++
	}
	return -1
}
