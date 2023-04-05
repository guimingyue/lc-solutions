#include <unordered_map>

using namespace std;

class Solution {
private:
    char arr[5] = {'a', 'e', 'i', 'o', 'u'};
    unordered_map<char, int> map = {{'a', 5}, {'e', 4}, {'i', 3}, {'o', 2}, {'u', 1}};
public:
    int countVowelStrings(int n) {
        int res = 0;
        for (int i = 0; i < 5; i++) {
            res += count(arr[i], n-1);
        }
        return res;
    }

    int count(char pre, int n) {
        if (n == 0) {
            return 1;
        }
        if (n == 1) {
            return map[pre];
        }
        int res = 0;
        for (int i = 0; i < 5; i++) {
            if (arr[i] < pre) {
                continue;
            }
            int v = count(arr[i], n-1);
            res += v;
        }
        return res;
    }
};