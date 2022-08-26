package leetcode

import (
	"strconv"
	"unicode"
)

func fractionAddition(expression string) string {
	abs := func(v int) int {
		if v < 0 {
			return -v
		}
		return v
	}
	gcd := func(x, y int) int {
		x, y = abs(x), abs(y)
		if x < y {
			x, y = y, x
		}
		for y != 0 {
			x, y = y, x%y
		}
		return x
	}
	readFractionNum := func(expr string, idx int) (int, int, int) {
		start := idx
		for idx < len(expr) && expr[idx] != '/' {
			idx++
		}
		numerator, _ := strconv.Atoi(expr[start:idx])
		idx++
		start = idx
		for idx < len(expr) && unicode.IsDigit(rune(expr[idx])) {
			idx++
		}
		denominator, _ := strconv.Atoi(expr[start:idx])
		return numerator, denominator, idx
	}
	numerator1, denominator1, idx := readFractionNum(expression, 0)
	for idx < len(expression) {
		numerator2, denominator2, i := readFractionNum(expression, idx)
		idx = i
		g := gcd(denominator1, denominator2)
		newDenominator := denominator1 / g * denominator2
		num1 := numerator1 * denominator2 / g
		num2 := numerator2 * denominator1 / g
		newNumerator := num1 + num2
		numerator1, denominator1 = newNumerator, newDenominator
	}
	g := gcd(numerator1, denominator1)
	v1 := numerator1 / g
	v2 := denominator1 / g

	return strconv.FormatInt(int64(v1), 10) + "/" + strconv.FormatInt(int64(v2), 10)
}
