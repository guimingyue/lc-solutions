#include "common.h"

using namespace std;

class Solution {
public:
    int res = 0;
    int maxAncestorDiff(TreeNode* root) {
        maxmin(root);
        return res;
    }

    pair<int, int> maxmin(TreeNode* root) {
        if (root->left == nullptr && root->right == nullptr) {
            return {root->val, root->val};
        }
        int minVal = 100001, maxVal = -1;
        if (root->left != nullptr) {
            pair<int, int> p = maxmin(root->left);
            minVal = min(minVal, p.first);
            maxVal = max(maxVal, p.second);
        }
       if (root->right != nullptr) {
            pair<int, int> p = maxmin(root->right);
            minVal = min(minVal, p.first);
            maxVal = max(maxVal, p.second);
        }
        res = max(res, max(abs(root->val - minVal), abs(root->val - maxVal)));
        return {min(root->val, minVal), max(root->val, maxVal)};
    }
};