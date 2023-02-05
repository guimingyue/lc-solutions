#include <algorithm>
#include <string>

using namespace std;

class Solution {
public:
    string getSmallestString(int n, int k) {
         int num = k / n;
        int mod = k % n;
        string res(n, 'a' + (num-1));
        int i = n - 1;
        while (mod > 0) {
            int d = min('z' - res[i], mod);
            res[i] += d;
            i--;
            mod -= d;
        }
        int l = 0, r = n - 1;
        while (l < r) {
            int d = min(res[l] - 'a', 'z' - res[r]);
            res[l] = res[l] - d;
            res[r] = res[r] + d;
            if (res[l] == 'a') {
                l++;
            }
            if (res[r] == 'z') {
                r--;
            }
        }
        return res;
    }
};