#include <queue>
#include <vector>

using namespace std;

struct Cmp1 {

    bool operator ()(const pair<int, int> &a, const pair<int, int> &b){
        return a.first >= b.first;
    }
};

struct Cmp2 {

    bool operator ()(const pair<int, int> &a, const pair<int, int> &b){
        return a.first <= b.first;
    }
};

class Solution {
public:
    int getNumberOfBacklogOrders(vector<vector<int>>& orders) {
        unsigned long long  res = 0;
        priority_queue<pair<int, int>, vector<pair<int,int>>, Cmp2> buy;
        priority_queue<pair<int, int>, vector<pair<int,int>>, Cmp1> sell;
        for (auto vec : orders) {
            if (vec[2] == 0) { // buy
                while (!sell.empty() && vec[1] > 0) {
                    auto top = sell.top();
                    if (vec[0] >= top.first) {
                        sell.pop();
                        if (vec[1] >= top.second) {
                            vec[1] -= top.second;
                            res -= top.second;
                        } else {
                            top.second -= vec[1];
                            res -= vec[1];
                            vec[1] = 0;
                            sell.push({top.first, top.second});
                        }
                    } else {
                        break;
                    }
                }
                if (vec[1] > 0) {
                    buy.push({vec[0], vec[1]});
                    res += vec[1];
                }
            } else { // sell
                while (!buy.empty() && vec[1] > 0) {
                    auto top = buy.top();
                    if (vec[0] <= top.first) {
                        buy.pop();
                        if (vec[1] >= top.second) {
                            vec[1] -= top.second;
                            res -= top.second;
                        } else {
                            top.second -= vec[1];
                            res -= vec[1];
                            vec[1] = 0;
                            buy.push({top.first, top.second});
                        }
                    } else {
                        break;
                    }
                }
                if (vec[1] > 0) {
                    sell.push({vec[0], vec[1]});
                    res += vec[1];
                }
            }
        }
        return res % 1000000007;
    }
};