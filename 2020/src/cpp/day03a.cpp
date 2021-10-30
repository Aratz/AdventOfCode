#include <iostream>
#include <string>
#include <vector>

int main() {
    std::vector<std::string> slope;

    std::string row;

    while(std::cin >> row)
        slope.push_back(row);

    int res = 0;
    for(int i = 0; i < slope.size(); i++)
        if(slope[i][i*3 % slope[i].size()] == '#')
            res += 1;

    std::cout << res << '\n';
    return 0;
}
