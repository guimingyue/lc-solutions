#include <string>

using namespace std;

class Solution {
public:
    string maskPII(string s) {
        char ch = s[0];
        string res;
        if ((ch >= 'A' && ch <= 'Z') || (ch >= 'a' && ch <= 'z')) {
            res.push_back(::tolower(ch));
            res.append("*****");
            bool atPassed = false;
            for (int i = 1; i < s.length(); i++) {
                if (!atPassed) {
                    if (s[i+1] == '@') {
                        res.push_back(::tolower(s[i]));
                    } else if (s[i] == '@') {
                        atPassed = true;
                        res.push_back(s[i]);
                    }
                } else {
                    res.push_back(::tolower(s[i]));
                }
            }
        } else {
            string digit;
            for (int i = 0; i < s.length(); i++) {
                if (s[i] >= '0' && s[i] <= '9') {
                    digit.push_back(s[i]);
                }
            }
            int digitCnt = digit.length();
            if (digitCnt == 10) {
                res.append("***-***-");
            } else if (digitCnt == 11) {
                res.append("+*-***-***-");
            } else if (digitCnt == 12) {
                res.append("+**-***-***-");
            } else if (digitCnt == 13) {
                res.append("+***-***-***-");
            }
            res.append(digit, digit.length()-4, 4);
        }
        return res;
    }
};