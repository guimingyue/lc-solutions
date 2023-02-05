#include <vector>

using namespace std;

class Solution {
public:
    int waysToMakeFair(vector<int>& nums) {
        int oddRSum = 0, evenRSum = 0;
        for (int i = 0; i < nums.size(); i++) {
            if (i % 2 == 0) {
                evenRSum += nums[i];
            } else {
                oddRSum += nums[i];
            }
        }
        int res = 0;
        int oddLSum = 0, evenLSum = 0;
        for (int i = 0; i < nums.size(); i++) {
            if (i % 2 == 0) {
                evenRSum -= nums[i];
            } else {
                oddRSum -= nums[i];
            }
            if (evenRSum + oddLSum == oddRSum + evenLSum) {
                res++;
            }
            if (i % 2 == 0) {
                evenLSum += nums[i];
            } else {
                oddLSum += nums[i];
            }
        }
        return res;
    }
};