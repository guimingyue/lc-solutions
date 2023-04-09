#include <string>
#include <bitset>

using namespace std;

class Solution {
public:
    string baseNeg2(int n) {
        if (n == 0) {
            return "0";
        }
         string s = bitset<32>(n).to_string();
        int i = 0;
        while (s[i] == '0') {
            i++;
        }
        s = s.substr(i);
        reverse(s.begin(), s.end());
        string res;
        bool plus = false;
        for (i = 0; i < s.size(); i++) {
            if (plus) {
                if (i % 2 == 0 && s[i] == '1') {
                    res.push_back('0');
                    plus = true;
                } else if (i % 2 == 0 && s[i] == '0') {
                    res.push_back('1');
                    plus = false;
                } else if (i % 2 != 0 && s[i] == '0') {
                    res.push_back('1');
                    plus = true;
                } else if (i % 2 != 0 && s[i] == '1') {
                    res.push_back('0');
                    plus = true;
                }
            } else {
                res.push_back(s[i]);
                if (i % 2 != 0 && s[i] == '1') {
                    plus = true;
                } else {
                    plus = false;
                }
            }
        }
        if (plus) {
            res.push_back('1');
            if (i % 2 != 0) {
                res.push_back('1');
            }
        }
        std::reverse(res.begin(), res.end());
        return res;
    }
};