#include <vector>
#include <regex>
#include <iostream>
#include <string>

std::string parse() {
    std::string s;
    std::string res = "";
    while(std::getline(std::cin, s)) {
        res += s;
    }
    return res;
}

int main() {
    auto input = parse();

    std::regex r(R"(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))");
    auto m_begin = std::sregex_iterator(input.begin(), input.end(), r);
    auto m_end = std::sregex_iterator();

    int sum = 0;
    bool enabled = true;

    for(auto it=m_begin; it != m_end; ++it) {
        std::smatch match = *it;
        if (match.str(0) == "do()") enabled = true;
        else if (match.str(0) == "don't()") enabled = false;
        else if (enabled) sum += std::stoi(match.str(1)) * std::stoi(match.str(2));
    }


    std::cout << "Solution A-part: " << sum << "\n";

    return 0;
}
