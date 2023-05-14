
#include <vector>
#include <unordered_set>

using namespace std;

class Solution {
public:
    int findMaxK(vector<int>& nums) {
        unordered_set<int> set;
        int res =  -1;
        for (auto num : nums) {
            if (set.count(-num) > 0) {
                res = max(res, num > 0 ? num : -num);
            }
            set.insert(num);
        }
        return res;
    }
};