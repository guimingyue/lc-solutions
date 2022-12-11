#include<vector>

using std::vector;

class Solution {
public:
    int minOperations(vector<int>& nums) {
        int res = 0;
        auto it = nums.begin();
        int last_val = *it;
        it++;
        while (it < nums.end()) {
            int val = *it;
            if (val <= last_val) {
                res += last_val + 1 - val;
                last_val += 1;
            } else {
                last_val = val;
            }
            it++;
        }
        return res;
    }
};