#include <vector>
#include <unordered_set>

using namespace std;

class Solution {
public:
    bool findSubarrays(vector<int>& nums) {
        unordered_set<int> set;
        for (int i = 0; i < nums.size()-1; i++) {
            int s = nums[i] + nums[i+1];
            if (set.count(s) > 0) {
                return true;
            }
            set.insert(s);
        }
        return false;
    }
};