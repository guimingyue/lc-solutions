#include <string>

using namespace std;
class Solution {
public:
    bool checkIfPangram(string sentence) {
        bool alphabet[26] = {false};
        for (char ch : sentence) {
            alphabet[ch - 'a'] = true;
        }
        bool res = true;
        for (bool exist : alphabet) {
            res &= exist;
        }
        return res;
    }
};