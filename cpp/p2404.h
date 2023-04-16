#include <vector>
#include <unordered_map>

using namespace std;

class Solution {
public:
    int mostFrequentEven(vector<int>& nums) {
        unordered_map<int, int> map;
        int res = -1, maxNum = 0;
        for (int num : nums) {
            if (num % 2 != 0) {
                continue;
            }
            map[num]++;
            if (map[num] > maxNum || (map[num] == maxNum && num < res)) {
                res = num;
                maxNum = map[num];
            }
        }
        return res;
    }
};