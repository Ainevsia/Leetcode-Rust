#include <string>
#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
#include <tuple>
#include <deque>
#include <unordered_map>
#include <map>
#include <cmath>
#include <queue>
using namespace std;

// I have no knowledge about what maps can do !
// think too much : no need to care about a->b and b->a
class UndergroundSystem {
public:
    map<pair<string, string>, int> time;
    map<pair<string, string>, int> cnt;
    map<int, string> instation;
    map<int, int> intime;

    UndergroundSystem() {
        time.clear();
        cnt.clear();
        instation.clear();
        intime.clear();
    }
    
    void checkIn(int id, string stationName, int t) {
        intime[id] = t;
        instation[id] = stationName;
    }
    
    void checkOut(int id, string stationName, int t) {
        time[make_pair(instation[id], stationName)] += t - intime[id];
        cnt[make_pair(instation[id], stationName)] ++ ;
    }
    
    double getAverageTime(string startStation, string endStation) {
        return time[make_pair(startStation, endStation)] / (double)
                cnt[make_pair(startStation, endStation)];
    }
};

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * UndergroundSystem* obj = new UndergroundSystem();
 * obj->checkIn(id,stationName,t);
 * obj->checkOut(id,stationName,t);
 * double param_3 = obj->getAverageTime(startStation,endStation);
 */

int main() {
    UndergroundSystem* undergroundSystem = new UndergroundSystem();
    undergroundSystem->checkIn(45, "Leyton", 3);
    undergroundSystem->checkIn(32, "Paradise", 8);
    undergroundSystem->checkIn(27, "Leyton", 10);
    undergroundSystem->checkOut(45, "Waterloo", 15);
    undergroundSystem->checkOut(27, "Waterloo", 20);
    undergroundSystem->checkOut(32, "Cambridge", 22);
    undergroundSystem->getAverageTime("Paradise", "Cambridge");       // return 14.0. There was only one travel from "Paradise" (at time 8) to "Cambridge" (at time 22)
    undergroundSystem->getAverageTime("Leyton", "Waterloo");          // return 11.0. There were two travels from "Leyton" to "Waterloo", a customer with id=45 from time=3 to time=15 and a customer with id=27 from time=10 to time=20. So the average time is ( (15-3) + (20-10) ) / 2 = 11.0
    undergroundSystem->checkIn(10, "Leyton", 24);
    undergroundSystem->getAverageTime("Leyton", "Waterloo");          // return 11.0
    undergroundSystem->checkOut(10, "Waterloo", 38);
    undergroundSystem->getAverageTime("Leyton", "Waterloo");          // return 12.0
    return 0;
}

