#include <iostream>
#include <vector>

int main() {
    int n;
    std::vector<int> numbers = std::vector<int>();
    while(std::cin >> n) {
        numbers.push_back(n);
    }

    int target = 2020;

    int res = -1;

    for(auto v1 : numbers) {
        for(auto v2: numbers) {
            if(v1 + v2 == target) {
                res = v1 * v2;
                break;
            }
        }
        if(res !=-1) break;
    }

    std::cout << res;
}
