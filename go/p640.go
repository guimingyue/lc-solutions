package leetcode

import (
	"strconv"
)

func solveEquation(equation string) string {
	leftXNum := 0
	leftNum := 0
	xNum := 0
	num := 0
	numFlag := 1
	curNum := 0
	for i, c := range equation {
		if c == '=' {
			leftXNum = xNum
			if equation[i-1] != 'x' {
				num += curNum * numFlag
			}
			leftNum = num
			xNum = 0
			num = 0
			curNum = 0
			numFlag = 1
			continue
		}
		if c == 'x' {
			if (i == 0 || equation[i-1] != '0') && curNum == 0 {
				curNum = 1
			}
			xNum += curNum * numFlag
			curNum = 0
			numFlag = 1
		} else if c == '-' {
			num += curNum * numFlag
			numFlag = -1
			curNum = 0
		} else if c == '+' {
			num += curNum * numFlag
			numFlag = 1
			curNum = 0
		} else {
			curNum = curNum*10 + (int(c) - int('0'))
		}
	}
	if equation[len(equation)-1] != 'x' {
		num += curNum * numFlag
	}
	if leftXNum == xNum && leftNum == num {
		return "Infinite solutions"
	}
	if leftXNum == xNum {
		return "No solution"
	}
	res := (num - leftNum) / (leftXNum - xNum)
	return "x=" + strconv.Itoa(res)
}
