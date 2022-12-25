#include <vector>

using namespace std;

class Solution {
public:
    int maximumScore(int a, int b, int c) {
        vector<int> vector;
        vector.push_back(a);
        vector.push_back(b);
        vector.push_back(c);
        std::sort(vector.begin(), vector.end());
        a = vector[0];
        b = vector[1];
        c = vector[2];
        if (a + b <= c) {
            return a + b;
        } else {
            int k = (a + b - c) / 2 + (a + b - c) % 2;;
            a = a - k;
            b = b - k;
            return k + a + b;
        }
    }
};