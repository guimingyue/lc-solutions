#include <string>

using namespace std;

class Solution {
public:
    int minimumMoves(string s) {
        int res = 0, len = 0, xnum = 0;
        for (int i = 0; i <= s.length(); i++) {
            char ch = i < s.length() ? s[i] : 'O';
            if (ch == 'X') {
                xnum++;
            }
            if (xnum > 0) {
                len++;
            }
            if (len == 3 || (i == s.length() && xnum > 0)) {
                res++;
                xnum = 0;
                len = 0;
            }
        }
        return res;
    }
};