/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    bool btreeGameWinningMove(TreeNode* root, int n, int x) {
        unordered_map<TreeNode*, int> map;
        TreeNode* xNode = nullptr;
        count(root, (unordered_map<TreeNode *, int> &) map, x, &xNode);
        if (xNode == root) {
            if (root->left == nullptr || root->right == nullptr) {
                return true;
            } else {
                int left = map[root->left];
                int right= map[root->right];
                if (left == right) {
                    return false;
                } else {
                    return true;
                }
            }
        } else {
            int cur = map[xNode];
        
            if (cur <= n/2) {
                return true;
            }
            if (xNode->left != nullptr) {
                int left = map[xNode->left];
                if (left > n/2) {
                    return true;
                }
            }
            if (xNode->right != nullptr) {
                int right = map[xNode->right];
                if (right > n/2) {
                    return true;
                }
            }
            return false;
        }
    }

    int count(TreeNode* root, unordered_map<TreeNode*, int> &map, int x, TreeNode** xNodePtr) {
        if (root == nullptr) {
            return 0;
        }
        if (root->val == x) {
            *xNodePtr = root;
        }
        int left = count(root->left, map, x, xNodePtr);
        int right = count(root->right, map, x, xNodePtr);
        int total = left + right + 1;
        map.insert({root, total});
        return total;
    }
};