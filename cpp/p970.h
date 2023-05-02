#include <vector>
#include <set>

using namespace std;

class Solution {
public:
    vector<int> powerfulIntegers(int x, int y, int bound) {
        int xp = 1;
        set<int> set;
        while (xp < bound) {
            int yp = 1;
            while (yp < bound) {
                int v = xp + yp;
                if (v <= bound) {
                    set.insert(v);
                }
                if (y == 1) {
                    break;
                }
                yp *= y;
            }
            if (x == 1) {
                break;
            }
            xp *= x;
        }
        vector<int> res(set.begin(), set.end());
        return res;
    }
};