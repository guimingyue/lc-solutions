#include <string>

using namespace std;

class Solution {
public:
    bool strongPasswordCheckerII(string password) {
        if (password.length() < 8) {
            return false;
        }
        bool containLowercase = false;
        bool containUppercase = false;
        bool containFlag = false;
        bool containDigit = false;
        char lastCh = ' ';
        for (int i = 0; i < password.length(); i++) {
            char ch = password[i];
            if (ch == '!' || ch == '@' || ch == '#' || ch == '$' || ch == '%'
            || ch == '^' || ch == '&' || ch == '*' || ch == '(' || ch == ')'
            || ch == '-' || ch == '+') {
                containFlag = true;
            } else if (ch >= 'a' && ch <= 'z') {
                containLowercase = true;
            } else if (ch >= 'A' && ch <= 'Z') {
                containUppercase = true;
            } else if(ch >= '0' && ch <= '9') {
                containDigit = true;
            } 
            if(ch == lastCh) {
                return false;
            }
            lastCh = ch;
        }
        return containLowercase && containUppercase && containFlag && containDigit;
    }
};