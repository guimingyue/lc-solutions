#include <string>
#include <unordered_set>

using namespace std;

class Solution {
public:
    bool equalFrequency(string word) {
         char chs[26] = {0};
        for (auto ch : word) {
            chs[ch-'a']++;
        }
        for (int i = 0; i < 26; i++) {
            if (chs[i] == 0) {
                continue;
            }
            chs[i]--;
            unordered_set<int> freq;
            for (int f : chs) {
                if (f > 0) {
                    freq.insert(f);
                }
            }
            if (freq.size() == 1) {
                return true;
            }
            chs[i]++;
        }
        return false;
    }
};