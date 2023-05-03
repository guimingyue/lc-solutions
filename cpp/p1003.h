#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    bool isValid(string s) {
        vector<char> stack;
        for (char ch : s) {
            if (ch == 'c') {
                int size = stack.size();
                if (size >= 2 && stack[size-1] == 'b' && stack[size-2] == 'a') {
                    stack.pop_back();
                    stack.pop_back();
                    continue;
                }
            }
            stack.push_back(ch);
        }
        return stack.size() == 0;
    }
};