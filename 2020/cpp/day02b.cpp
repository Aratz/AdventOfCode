#include<iostream>
#include<string>
#include<regex>
#include<vector>
#include<algorithm>

class PasswordPolicy {
    public:
        int min;
        int max;
        char letter;
        std::string password;

        PasswordPolicy(int min, int max, char letter, std::string password):
            min(min), max(max), letter(letter), password(password) {};
};

int main() {
    std::vector<std::string> lines;

    std::string line;
    while(getline(std::cin, line))
        lines.push_back(line);

    std::vector<PasswordPolicy> ppolicies;
    for(auto line: lines){
        auto re = std::regex("(\\d+)-(\\d+) ([a-z]): ([a-z]+)");
        std::match_results<std::string::const_iterator> mr;
        std::regex_search(line, mr, re);
        ppolicies.push_back(PasswordPolicy(
                    stoi(mr[1]),
                    stoi(mr[2]),
                    std::string(mr[3])[0],
                    mr[4]));
    }

    int res = 0;
    for(auto pp: ppolicies) {
        if((pp.password[pp.min-1] == pp.letter) != (pp.password[pp.max-1] == pp.letter))
            res += 1;
    }

   std::cout << res << '\n';

    return 0;
}
