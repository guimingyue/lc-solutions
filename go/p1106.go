package leetcode

func parseBoolExpr(expression string) bool {
	return eval(expression)
}

func eval(expr string) bool {
	token := next(expr)
	if token == 't' {
		return true
	} else if token == 'f' {
		return false
	} else if token == '!' {
		return !eval(expr[1:])
	} else if token == '|' {
		return or(expr[1:])
	} else if token == '(' {
		return eval(expr[1 : len(expr)-1])
	} else {
		return and(expr[1:])
	}
}

func next(expr string) uint8 {
	return expr[0]
}

func and(expr string) bool {
	return evalMulti(expr[1:len(expr)-1], true, func(val, next bool) bool {
		return val && next
	})
}

func or(expr string) bool {
	return evalMulti(expr[1:len(expr)-1], false, func(val, next bool) bool {
		return val || next
	})
}

func evalMulti(expr string, init bool, f func(val, next bool) bool) bool {
	bracketNum := 0
	res := init
	start := 0
	for i, ch := range expr {
		if ch == '(' {
			bracketNum++
		} else if ch == ')' {
			bracketNum--
		} else if ch == ',' && bracketNum == 0 {
			res = f(res, eval(expr[start:i]))
			start = i + 1
		}
	}
	return f(res, eval(expr[start:]))
}
