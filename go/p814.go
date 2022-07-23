func pruneTree(root *TreeNode) *TreeNode {
	n, _ := prune(root)
	return n
}

/**
* root node, all 0
 */
func prune(root *TreeNode) (*TreeNode, bool) {
	if root == nil {
		return nil, true
	}
	_, leftAllZero := prune(root.Left)
	if leftAllZero {
		root.Left = nil
	}
	_, rightAllZero := prune(root.Right)
	if rightAllZero {
		root.Right = nil
	}
	if leftAllZero && rightAllZero && root.Val == 0 {
		return nil, true
	}
	return root, false
}
