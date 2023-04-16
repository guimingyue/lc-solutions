#include <string>

using namespace std;

class Solution {
public:
    bool isRobotBounded(string instructions) {
        int direction[4][2] = {{0, 1}, {-1, 0},{0, -1},  {1, 0}};
        int x = 0, y = 0, d = 0;

        for (char ins : instructions) {
            if (ins == 'G') {
                x += direction[d][0];
                y += direction[d][1];
            } else if (ins == 'L') {
                d += 1;
                d %= 4;
            } else {
                d += 3;
                d %= 4;
            }
        }
        return d != 0 || (x == 0 && y == 0);
    }
};