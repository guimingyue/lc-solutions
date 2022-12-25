#include <string>

using namespace std;

class Solution {
public:
    string largestMerge(string word1, string word2) {
        string merge;
        int i = 0, j = 0;
        while (i < word1.length() || j < word2.length()) {
            if (i < word1.length() && word1.substr(i) > word2.substr(j)) {
                merge.push_back(word1[i++]);
            } else {
                merge.push_back(word2[j++]);
            }
        }
        return merge;
    }
};