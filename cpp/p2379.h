#include <string>

using namespace std;

class Solution {
public:
    int minimumRecolors(string blocks, int k) {
        int wnum = 0;
        int i = 0;
        while (i < k) {
            char ch = blocks[i++];
            if (ch == 'W') {
                wnum++;
            }
        }
        int res = wnum;
        for (; i < blocks.size(); i++) {
            char chb = blocks[i-k];
            char chn = blocks[i];
            if (chb == chn) {
                continue;
            } else if (chn == 'W') {
                wnum++;
            } else {
                wnum--;
            }
            res = min(res, wnum);
        }
        return res;
    }
};