#include <string>

using namespace std;

class Solution {
public:
    int rearrangeCharacters(string s, string target) {
        int a[26] = {0};
        for (auto c : s) {
            a[c - 'a']++;
        }
        int res = 0;
        while (true) {
        for (auto c : target) {
            if (a[c - 'a'] == 0) {
                return res;
            }
            a[c - 'a']--;
        }
        res++;
        }
        return res;
    }
};
