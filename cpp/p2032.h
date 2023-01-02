#include <vector>

using namespace std;

class Solution {
public:
    vector<int> twoOutOfThree(vector<int>& nums1, vector<int>& nums2, vector<int>& nums3) {
        vector<int> e1(100, 0), e2(100, 0), e3(100, 0);
        for (auto v : nums1) {
            e1[v-1] = 1;
        }
        for (auto v : nums2) {
            e2[v-1] = 1;
        }
        for (auto v : nums3) {
            e3[v-1] = 1;
        }
        vector<int> res;
        for (int i = 0; i < e1.size(); i++) {
            if (e1[i] + e2[i] + e3[i] >= 2) {
                res.push_back(i+1);
            }
        }
        return res;
    }
};