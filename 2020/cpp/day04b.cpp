#include <iostream>
#include <sstream>
#include <string>
#include <regex>
#include <vector>
#include <algorithm>

int main() {
    std::vector<std::string> mandatory_fields { "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" };

    std::string line;
    std::string passports;

    while(getline(std::cin, line)) {
        passports.append(line + "\n");
    }

    auto re = std::regex(R"!((?:ecl:([a-z]{3}|#[a-f0-9]+)\s|pid:(#?\w+)\s|eyr:(\d+)\s|hcl:(#?\w+)\s|byr:(\d+)\s|iyr:(\d+)\s|cid:(\d+)\s|hgt:(\d+(?:in|cm)?\s))+)!");

    int res = 0;
    for(
            auto i = std::sregex_iterator(passports.begin(), passports.end(), re);
            i != std::sregex_iterator();
            ++i) {
        auto m = *i;
        std::istringstream fields(m[0]);
        std::string field;
        int n_mandatory_fields = 0;
        while(fields >> field)
            if(std::find(
                    mandatory_fields.begin(),
                    mandatory_fields.end(),
                    field.substr(0, 3)) != mandatory_fields.end())
                n_mandatory_fields += 1;

        if(n_mandatory_fields == mandatory_fields.size())
            res += 1;
    }

    std::cout << res << '\n';
    return 0;
}
