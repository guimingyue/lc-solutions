func addDigits(num int) int {
	res := num
	for res >= 10 {
		temp := 0
		for res > 0 {
			temp += res % 10
			res = res / 10
		}
		res = temp
	}
	return res
}