package leetcode

func countStudents(students []int, sandwiches []int) int {
	for len(students) > 0 {
		if students[0] == sandwiches[0] {
			students = students[1:]
			sandwiches = sandwiches[1:]
		} else {
			i := 0
			for ; i < len(students); i++ {
				if students[i] == sandwiches[0] {
					break
				}
			}
			if i == len(students) {
				return i
			} else {
				students = append(students, students[:i]...)
				students = students[i:]
			}
		}
	}
	return 0
}
