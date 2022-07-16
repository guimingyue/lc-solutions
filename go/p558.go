/**
 * Definition for a QuadTree node.
 * type Node struct {
 *     Val bool
 *     IsLeaf bool
 *     TopLeft *Node
 *     TopRight *Node
 *     BottomLeft *Node
 *     BottomRight *Node
 * }
 */

 func intersect(quadTree1 *Node, quadTree2 *Node) *Node {
    if quadTree1.IsLeaf {
		if quadTree1.Val {
			return &Node{Val: true, IsLeaf: true}
		}
		return quadTree2
	}
	if quadTree2.IsLeaf {
		return intersect(quadTree2, quadTree1)
	}
	topLeft := intersect(quadTree1.TopLeft, quadTree2.TopLeft)
	topRight := intersect(quadTree1.TopRight, quadTree2.TopRight)
	bottomLeft := intersect(quadTree1.BottomLeft, quadTree2.BottomLeft)
	bottomRight := intersect(quadTree1.BottomRight, quadTree2.BottomRight)
	if topLeft.IsLeaf && topRight.IsLeaf && bottomLeft.IsLeaf && bottomRight.IsLeaf && topLeft.Val == topRight.Val &&
		topLeft.Val == bottomLeft.Val && topLeft.Val == bottomRight.Val {
		return &Node{Val: topLeft.Val, IsLeaf: true}
	}
	return &Node{Val: false, IsLeaf: false, TopLeft: topLeft, TopRight: topRight, BottomLeft: bottomLeft, BottomRight: bottomRight}
}