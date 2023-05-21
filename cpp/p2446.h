#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    bool haveConflict(vector<string>& event1, vector<string>& event2) {
        if (event1[0] <= event2[0]) {
            return isConflict(event1, event2);
        } else {
            return isConflict(event2, event1);
        }
    }
    bool isConflict(vector<string>& event1, vector<string>& event2) {
        return (event1[0] <= event2[0] && event2[1] <= event1[1])
        || (event2[0] <= event1[1] && event1[1] <= event2[1]);
    }
};