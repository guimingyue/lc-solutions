#include <vector>

using namespace std;

class Solution {
public:
    vector<int> circularPermutation(int n, int start) {
        vector<int> res;
        res.reserve(1 << n);
        res.push_back(start);
        for (int i = 1; i <= n; i++) {
            int m = res.size();
            for (int j = m-1; j >= 0; j--) {
                res.push_back(((res[j] ^ start) | (1 << (i-1))) ^ start);
            }
        }
        return res;
    }
};