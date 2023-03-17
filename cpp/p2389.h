#include <vector>

using namespace std;

class Solution {
public:
    vector<int> answerQueries(vector<int>& nums, vector<int>& queries) {
        sort(nums.begin(), nums.end());
        vector<int> sums;
        int sum = 0;
        for (auto num : nums) {
            sum += num;
            sums.push_back(sum);
        }
        vector<int> res;
        for (auto q : queries) {
            auto up = upper_bound(sums.begin(), sums.end(), q);
            int idx = up - sums.begin();
            res.push_back(idx);
        }
        return res;
    }
};