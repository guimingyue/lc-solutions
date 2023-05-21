#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
public:
    int numTilePossibilities(string tiles) {
        unordered_map<char, int> cnt;
        vector<char> tile;
        for (char ch : tiles) {
            cnt[ch]++;
            if (cnt[ch] == 1) {
                tile.push_back(ch);
            }
        }
        return dfs(cnt, tile, tiles.size()) - 1;
    }

    int dfs(unordered_map<char, int> cnt, vector<char> tile, int i) {
        if (i == 0) {
            return 1;
        }
        int res = 1;
        for (char ch : tile) {
            if (cnt[ch] > 0) {
                cnt[ch]--;
                res += dfs(cnt, tile, i - 1);
                cnt[ch]++;
            }
        }
        return res;
    }
};