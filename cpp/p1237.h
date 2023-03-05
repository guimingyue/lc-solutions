#include <vector>

using namespace std;

class CustomFunction {
public:
 int f(int x, int y);
 };

class Solution {
public:
    vector<vector<int>> findSolution(CustomFunction& customfunction, int z) {
        vector<vector<int>> res;
        for (int x = 1; x <= 1000; x++) {
            for (int y = 1; y <= 1000; y++) {
                if (customfunction.f(x, y) == z) {
                    res.push_back({x, y});
                }
            }
        }
        return res;
    }
};