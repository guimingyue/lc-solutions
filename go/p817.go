package leetcode

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */

func numComponents(head *ListNode, nums []int) int {
	cnt := make(map[int]bool)
	for _, v := range nums {
		cnt[v] = true
	}
	res := 0
	p := head
	for p != nil {
		if cnt[p.Val] {
			for p != nil && cnt[p.Val] {
				p = p.Next
			}
			res++
		}
		if p == nil {
			break
		}
		p = p.Next
	}
	return res
}
