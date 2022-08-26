package leetcode

func generateTheString(n int) string {
	arr := make([]byte, 26)
	for i := 0; i < 26; i++ {
		arr[i] = 'a' + byte(i)
	}
	bytes, _ := generate(n, arr, 0)
	return string(bytes)
}

func generate(n int, arr []byte, idx int) ([]byte, int) {
	if n%2 != 0 {
		bytes := make([]byte, n)
		for i := 0; i < n; i++ {
			bytes[i] = arr[idx]
		}
		idx++
		return bytes, idx
	}
	next := n / 2
	if next%2 == 0 {
		next++
	}
	bytes1, idx1 := generate(next, arr, idx)
	bytes2, idx2 := generate(n-next, arr, idx1)
	return append(bytes1, bytes2...), idx2
}
