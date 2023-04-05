#include <string>

using namespace std;

class Solution {
public:
    int countSubstrings(string s, string t) {
        int m = s.size(), n = t.size();
        int res = 0;
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                int diff = 0;
                for (int k = 0; i + k < m && j + k < n; k++) {
                    diff += s[i+k] == t[j+k] ? 0 : 1;
                    if (diff > 1) {
                        break;
                    } else if (diff == 1) {
                        res++;
                    }
                }
            }
        }
        return res;
    }
};