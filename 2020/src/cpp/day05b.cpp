#include <vector>
#include <algorithm>
#include <string>
#include <vector>
#include <iostream>

using namespace std;

int main(void) {
    vector<string> lines;
    {
        string line;
        while(getline(cin, line)) lines.push_back(line);
    }

    vector<int> codes;
    for(auto s: lines){
        int code = 0;
        for(int i = 0; i < s.size(); i++)
            code += (s[i] == 'R' || s[i] == 'B'?1:0) << (s.size() - i - 1);

        codes.push_back(code);
    }

    sort(codes.begin(), codes.end());

    for(int i = 0; i < codes.size() - 1; i++){
        if (codes[i] != codes[i+1] - 1) {
            cout << (codes[i] + 1) << "\n";
            break;
        }
    }



    return 0;
}
