#include <vector>
#include <stack>
#include "common.h"

using namespace std;

class Solution {
public:
    vector<int> nextLargerNodes(ListNode* head) {
        vector<int> res;
        stack<pair<int, int>> s;
        ListNode* cur = head;
        int idx = -1;
        while (cur != nullptr) {
            idx++;
            res.push_back(0);
            while (!s.empty() && s.top().first < cur->val) {
                res[s.top().second] = cur->val;
                s.pop();
            }
            s.emplace(cur->val, idx);
            cur = cur->next;
        }
        return res;
    }
};