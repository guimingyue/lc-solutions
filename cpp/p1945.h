#include <string>

using namespace std;

class Solution {
public:
    int getLucky(string s, int k) {
        int init = 0;
        for (auto ch : s) {
            auto num = ch - 'a' + 1;
            if (num < 10) {
                init += num;
            } else {
                init += num % 10;
                init += (num / 10) % 10;
            }
        }
        k--;
        int num = init;
        while (k-- > 0) {
            int newNum = 0;
            while (num > 0) {
                newNum += num % 10;
                num = num / 10;
            }
            num = newNum;
        }
        return num;
    }
};