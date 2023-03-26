#include <vector>

using namespace std;

class Solution {
public:
    vector<bool> checkArithmeticSubarrays(vector<int>& nums, vector<int>& l, vector<int>& r) {
        vector<bool> res;
        for (int i = 0; i < l.size(); i++) {
            vector<int> v;
            for (int j = l[i]; j <= r[i]; j++) {
                v.push_back(nums[j]);
            }
            sort(v.begin(), v.end());
            int d = v[1] - v[0];
            bool diff = true;
            for (int j = 2; j < v.size(); j++) {
                if (v[j] - v[j-1] != d) {
                    diff = false;
                    break;
                }
            }
        
            res.push_back(diff);
        
        }
        return res;
    }
};