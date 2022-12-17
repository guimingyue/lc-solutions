#include <vector>

using namespace std;

class Solution {
public:
    int minElements(vector<int>& nums, int limit, int goal) {
        long long sum = 0;
        for (auto it = nums.begin(); it < nums.end(); it++) {
            sum += *it;
        }
        long long left = abs(goal - sum);
        return left / limit + (left % limit == 0 ? 0 : 1);
        
    }
};