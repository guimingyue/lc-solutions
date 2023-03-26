#include <string>

using namespace std;

class Solution {
public:
    int minimumDeletions(string s) {
        int righta = 0, leftb = 0;
        for (auto ch : s) {
            if (ch == 'a') {
                righta++;
            }
        }
        int res = righta;
        for (int i = 0; i < s.size(); i++) {
            if (s[i] == 'a') {
                righta--;
            } else {
                leftb++;
            }
            res = min(res, righta + leftb);
        }
        return res;
    }
};