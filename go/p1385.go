func findTheDistanceValue(arr1 []int, arr2 []int, d int) int {
	res := 0
	sort.Ints(arr2)
	for idx := 0; idx < len(arr1); idx++ {
		i := binary_search(arr2, arr1[idx])
		if i > 0 && math.Abs(float64(arr2[i - 1] - arr1[idx])) <= float64(d) {
			continue
		}
		if math.Abs(float64(arr2[i] - arr1[idx])) <= float64(d) {
			continue
		}

		if i < len(arr2) - 1 && math.Abs(float64(arr2[i + 1] - arr1[idx])) <= float64(d) {
			continue
		}

		res ++
	}
	return res
}

func binary_search(arr []int, v int) int {
	var i, j = 0, len(arr) - 1
	for i < j {
		mid := i + (j - i) / 2
		if v < arr[mid] {
			j = mid - 1
		} else if v > arr[mid] {
			i = mid + 1
		} else {
			return mid
		}
	}
	return i
}