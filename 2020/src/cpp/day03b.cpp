#include <iostream>
#include <string>
#include <vector>

int main() {
    std::vector<std::string> slope;

    std::string row;

    while(std::cin >> row)
        slope.push_back(row);

    size_t res = 1;

    std::vector<int> angles {1, 3, 5, 7};

    for(auto angle: angles){
        int partial_res = 0;
        for(int i = 0; i < slope.size(); i++)
            if(slope[i][i*angle % slope[i].size()] == '#')
                partial_res += 1;
        res *= partial_res;
    }

    int partial_res = 0;
    for(int i = 0; i < slope.size(); i += 2){
        if(slope[i][i/2 % slope[i].size()] == '#')
            partial_res += 1;
    }
    res *= partial_res;

    std::cout << res << '\n';
    return 0;
}
