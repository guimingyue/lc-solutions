#include <vector>

using namespace std;

class Solution {
public:
    vector<int> numMovesStones(int a, int b, int c) {
        vector<int> array;
        array.push_back(a);
        array.push_back(b);
        array.push_back(c);
        sort(array.begin(), array.end());
        int minTime = (array[2] - array[1] - 1 > 0 ? 1 : 0) + (array[1] - array[0] - 1 > 0 ? 1 : 0);
        if (array[2] - array[1] == 2 || array[1] - array[0] == 2) {
            minTime = min(minTime, 1);
        }
        int maxTime = array[2] - array[0] - 2;
        return {minTime, maxTime};
    }
};