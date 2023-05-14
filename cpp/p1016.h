#include <string>

using namespace std;

class Solution {
public:
    bool queryString(string s, int n) {
        string str;
        for (int i = 1; i <= n; i++) {
            addOne(str);
            int idx = s.find(str);
            if (idx < 0) {
                return false;
            }
        }
        return true;
    }

    void addOne(string &s) {
        int carry = 1;
        for (int i = s.length() - 1; i >= 0; i--) {
            char ch = s[i];
            if (ch == '0') {
                s[i] = '1';
                carry = 0;
                break;
            } else {
                s[i] = '0';
            }
        }
        if (carry == 1) {
            s.insert(0, 1,'1');
        }
    }
};