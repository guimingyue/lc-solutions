#include <vector>
#include <unordered_map>
#include <queue>

using namespace std;

struct Cmp {

    bool operator ()(const pair<int, int> &a, const pair<int, int> &b){
        return a.second <= b.second;
    }
};
class Solution {
public:
    vector<int> rearrangeBarcodes(vector<int>& barcodes) {
        unordered_map<int, int> map;
        for (auto v : barcodes) {
            map[v]++;
        }
        priority_queue<pair<int, int>, vector<pair<int, int>>, Cmp> queue;
        for (auto v : map) {
            queue.emplace(v);
        }
        vector<int> res(barcodes.size(), 0);
        int i = 0;
        while (!queue.empty()) {
            pair<int, int> p = queue.top();
            queue.pop();
            for (int j = 0; j < p.second; j++) {
                res[i] = p.first;
                i += 2;
                if (i >= barcodes.size()) {
                    i = 1;
                }
            }
        }
        return res;
    }
};