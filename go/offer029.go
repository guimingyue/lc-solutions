/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Next *Node
 * }
 */

 func insert(aNode *Node, x int) *Node {
    if aNode == nil {
		aNode = &Node{Val: x}
		aNode.Next = aNode
	} else {
		p := aNode
		max := p
		for x < p.Val || x > p.Next.Val {
			if p.Next == aNode {
				break
			}
			p = p.Next
			if max.Val <= p.Val {
				max = p
			}
		}
		if x >= p.Val && x <= p.Next.Val {
			pNext := p.Next
			p.Next = &Node{Val: x}
			p.Next.Next = pNext
		} else {
			pNext := max.Next
			max.Next = &Node{Val: x}
			max.Next.Next = pNext
		}
	}
	return aNode
}