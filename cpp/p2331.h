#include "common.h"

class Solution {
public:
    bool evaluateTree(TreeNode* root) {
        if (root->left == nullptr && root->right == nullptr) {
            return root->val == 0 ? false : true;
        }
        bool left = evaluateTree(root->left);
        bool right = evaluateTree(root->right);
        if (root->val == 2) {
            return left | right;
        } else {
            return left & right;
        }
    }
};