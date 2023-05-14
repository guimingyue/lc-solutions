
#include <unordered_set>

using namespace std;

class Solution {
public:
    int smallestRepunitDivByK(int k) {
        int resid = 1 % k, len = 1;
        unordered_set<int> set;
        set.insert(resid);
        while (resid != 0) {
            resid = (resid * 10 + 1) % k;
            if (set.find(resid) != set.end()) {
                return -1;
            }
            set.insert(resid);
            len++;
        }
        return len;
    }
};