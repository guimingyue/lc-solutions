#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int prefixCount(vector<string>& words, string pref) {
        int res = 0;
        for (auto word : words) {
            size_t found = word.find(pref);
            if (found == 0) {
                res++;
            }
        }
        return res;
    }
};