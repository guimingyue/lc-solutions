#include <string>
#include <unordered_map>

using namespace std;

class AuthenticationManager {
private:
    unordered_map<string, int> map;
    int timeToLive;

public:
    AuthenticationManager(int timeToLive): timeToLive(timeToLive) {

    }

    void generate(string tokenId, int currentTime) {
        map.insert({tokenId, currentTime + timeToLive});
    }

    void renew(string tokenId, int currentTime) {
        if (map.count(tokenId) <= 0) {
            return;
        }
        if (map[tokenId] <= currentTime) {
            map.erase(tokenId);
            return;
        }
        map.erase(tokenId);
        map.insert({tokenId, currentTime + timeToLive});
    }

    int countUnexpiredTokens(int currentTime) {
        int res = 0;
        for (auto [_, expireTime] : map) {
            if (expireTime > currentTime) {
                res++;
            }
        }
        return res;
    }
};

/**
 * Your AuthenticationManager object will be instantiated and called as such:
 * AuthenticationManager* obj = new AuthenticationManager(timeToLive);
 * obj->generate(tokenId,currentTime);
 * obj->renew(tokenId,currentTime);
 * int param_3 = obj->countUnexpiredTokens(currentTime);
 */