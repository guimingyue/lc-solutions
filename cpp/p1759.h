#include <string>

using namespace std;

class Solution {
public:
    int countHomogenous(string s) {
        const int MOD = 1000000007;
        int len = 1;
        int res = 0;
        for (int i = 1; i <= s.size(); i++) {
            if (i == s.size() || s[i] != s[i-1]) {
                unsigned long long delta = (1ull + len) * len / 2;
                res = (res + delta) % MOD;
                len = 1;
            } else {
                len++;
            }
        }
        return res;
    }
};