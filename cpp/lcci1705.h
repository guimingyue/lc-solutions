#include <vector>
#include <string>
#include <unordered_map>

using namespace std;

class Solution {
public:
    vector<string> findLongestSubarray(vector<string>& array) {
        int start = 0, end = -1, sum = 0;
        unordered_map<int, int> map;
        map[0] = -1;
        for (int i = 0; i < array.size(); i++) {
            sum += digit(array[i][0]);
            if (map.count(sum) > 0) {
                if (i - map[sum] > end - start) {
                    start = map[sum];
                    end = i;
                }
            } else {
                map[sum] = i;
            }
        }
        vector<string> res;
        for (int i = start+1; i <= end; i++) {
            res.push_back(array[i]);
        }
        return res;
    }
    int digit(char ch) {
        if (::isdigit(ch)) {
            return 1;
        }
        return -1;
    }
};