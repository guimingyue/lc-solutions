#include <string>

using namespace std;

class Solution {
public:
    char repeatedCharacter(string s) {
        bool a[26] = {false};
        for (auto c : s) {
            if (a[c - 'a']) {
                return c;
            }
            a[c - 'a'] = true;
        }
        return s[0];
    }
};