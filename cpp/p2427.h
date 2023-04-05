#include <algorithm>

using namespace std;
class Solution {
public:
    int commonFactors(int a, int b) {
        int res = 1;
        for (int v = 2; v <= min(a, b); v++) {
            if (a % v == 0 && b % v == 0) {
                res++;
            }
        }
        return res;
    }
};