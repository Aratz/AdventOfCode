#include <vector>
#include <iostream>
#include <sstream>
#include <cmath>

std::vector<std::vector<int>> parse() {
    std::vector<std::vector<int>> reports;
    std::string s;
    while (std::getline(std::cin, s)) {
        std::vector<int> report;
        std::stringstream ss(s);
        int n;
        while(ss >> n) {
            report.push_back(n);
        }
        reports.push_back(report);
    }

    return reports;
}

bool is_safe(const std::vector<int>& report, int tol) {
    if (tol) {
        for (int skip = 0; skip < report.size(); ++skip) {
            bool safe = true;
            int inc;
            if (skip == 0) {
                inc = report[1] < report[2] ? 1 : -1;
            }
            else if (skip == 1) {
                inc = report[0] < report[2] ? 1 : -1;
            }
            else {
                inc = report[0] < report[1] ? 1 : -1;
            }

            for (int i=0; i < report.size() - 1; ++i) {
                if (i == skip) { continue; }
                if (i + 1 == skip && skip == report.size() - 1) { continue; }
                int next = i + 1 != skip ? i + 1 : i + 2;
                if (
                    inc*report[i] >= inc*report[next]
                    || 3 < inc*(report[next] - report[i])
                ) {
                        safe = false; break;
                }
            }

            if (safe) {
                return true;
            }
        }
        return false;
    }
    else {
        int inc = report[0] < report[1] ? 1 : -1;
        for (int i=0; i < report.size() - 1; ++i) {
            if (
                inc*report[i] >= inc*report[i + 1]
                || 3 < inc*(report[i + 1] - report[i])
            ) {
                return false;
            }
        }
    return true;
    }
}

int solve(const std::vector<std::vector<int>>& reports, int tol) {
    int n_safe = 0;
    for (const auto& report: reports) {
        n_safe += is_safe(report, tol) ? 1 : 0;
    }

    return n_safe;
}

int main() {

    auto input = parse();

    std::cout << "Solution A-part: " << solve(input, 0) << "\n";
    std::cout << "Solution B-part: " << solve(input, 1) << "\n";

    return 0;
}
