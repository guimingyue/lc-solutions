
#include <queue>
#include <set>

using namespace std;

struct Cmp {

    bool operator ()(const pair<int, int> &a, const pair<int, int> &b){
        int d1 = a.second - a.first, d2 = b.second - b.first;
        if (d1 / 2 < d2 / 2) {
            return true;
        } else if (d1 / 2 == d2 / 2){
            return a.first > b.first;
        } else {
            return false;
        }
    }
};

class ExamRoom {
private:
    int n;
    set<int> seats;
    priority_queue<pair<int, int>, vector<pair<int, int>>, Cmp> queue;

public:
    ExamRoom(int n): n(n) {

    }

    int seat() {
        if (seats.empty()) {
            seats.insert(0);
            return 0;
        }

        int left = *seats.begin(), right = n - 1 - *seats.rbegin();
        while (seats.size() >= 2) {
            pair<int, int> top = queue.top();
            if (seats.count(top.first) > 0 && seats.count(top.second) > 0 &&
            *next(seats.find(top.first)) == top.second) {
                int d = top.second - top.first;
                if (d/2 <= left || d / 2 < right) {
                    break;
                }
                queue.pop();
                queue.push({top.first, top.first + d/2});
                queue.push({top.first + d/2, top.second});
                seats.insert(top.first + d/2);
                return top.first + d / 2;
            }
            queue.pop();
        }
        if (right > left) {
            queue.push({*seats.rbegin(), n-1});
            seats.insert(n-1);
            return n-1;
        } else {
            queue.push({0, *seats.begin()});
            seats.insert(0);
            return 0;
        }
    }

    void leave(int p) {
        if (p != *seats.begin() && p != *seats.rbegin()) {
            auto it = seats.find(p);
            queue.push({*prev(it), *next(it)});
        }
        seats.erase(p);
    }
};
