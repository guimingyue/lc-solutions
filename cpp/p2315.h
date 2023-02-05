#include <string>

using namespace std;

class Solution {
public:
    int countAsterisks(string s) {
        bool in = false;
        int res = 0;
        for (char ch : s) {
            if (ch == '|') {
                if (!in) {
                    in = true;
                } else {
                    in = false;
                }
            } else if (!in && ch == '*') {
                res++;
            }
        }
        return res;
    }
};