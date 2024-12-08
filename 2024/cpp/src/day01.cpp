#include <vector>
#include <algorithm>
#include <cstdlib>
#include <iostream>
#include <unordered_map>

int solve_a(const std::vector<int>& v1, const std::vector<int>& v2) {
    int total_diff = 0;
    for(int i=0; i < v1.size(); ++i) {
        total_diff += std::abs(v1[i] - v2[i]);
    }

    return total_diff;
}

int solve_b(const std::vector<int>& v1, const std::vector<int>& v2) {
    std::unordered_map<int, int> counts;
    for (const auto &n : v2) {
        if (counts.find(n) == counts.end()) {
            counts[n] = 0;
        }
        counts[n] += 1;
    }

    int total = 0;
    for (const auto &n : v1) {
        total += counts.find(n) != counts.end() ? counts[n] * n : 0;
    }

    return total;
}

int main() {
    std::vector<int> v1, v2;
    int n1, n2;
    while (std::cin >> n1 >> n2) {
        v1.push_back(n1);
        v2.push_back(n2);
    }

    std::sort(v1.begin(), v1.end());
    std::sort(v2.begin(), v2.end());

    std::cout << "Solution A-part: " << solve_a(v1, v2) << "\n";
    std::cout << "Solution B-part: " << solve_b(v1, v2) << "\n";

    return 0;
}
