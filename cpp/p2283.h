#include <string>

using namespace std;

class Solution {
public:
    bool digitCount(string num) {
         int a[10] = {0};
        for (auto s : num) {
            a[s-'0']++;
        }
        for (int i = 0; i < num.length(); i++) {
            if ((num[i] - '0') != a[i]) {
                return false;
            }
        }
        return  true;
    }
};