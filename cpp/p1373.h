#include "common.h"

using namespace std;

class Solution {
public:
    int res;
    struct SubTree {
        bool isBst;
        int minVal;
        int maxVal;
        int sumVal;
        SubTree(bool isBst, int minVal, int maxVal, int sumVal): isBst(isBst), minVal(minVal), maxVal(maxVal), sumVal(sumVal) {

        }
    };
    int maxSumBST(TreeNode* root) {
        res = 0;
        maxSumDfs(root);
        return res;
    }

    SubTree maxSumDfs(TreeNode* node) {
        if (node == nullptr) {
            return {true, INT_MAX, INT_MIN, 0};
        }
        auto left = maxSumDfs(node->left);
        auto right = maxSumDfs(node->right);

        if (left.isBst && right.isBst && left.maxVal < node->val && right.minVal > node->val) {
            int sum = node->val + left.sumVal + right.sumVal;
            res = max(res, sum);
            return {true, min(node->val, left.minVal), max(node->val, right.maxVal), sum};
        } else {
            return {false, 0, 0, 0};
        }
    }
};