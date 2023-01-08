#include <string>

using namespace std;

class Solution {
public:
    bool areNumbersAscending(string s) {
        int p = 0;
        int pre = 0;
        while (p < s.length()) {
            int num = next(s, &p);
            if (num == -1) {
                continue;
            }
            if (num <= pre) {
                return false;
            }
            pre = num;
        }
        return true;
    }
private:
    int next(string s, int *p) {
        int i = *p;
        while (i < s.length() && (s[i] < '0' || s[i] > '9' || s[i] == ' ')) {
            i++;
        }
        int num = 0;
        while (i < s.length() && s[i] >= '0' && s[i] <= '9') {
            num = num * 10 + (s[i] - '0');
            i++;
        }
        *p = i;
        return num == 0 ? -1 : num;
    }
};