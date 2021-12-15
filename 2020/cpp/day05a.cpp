#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

int main(int argc, char* argv[]){
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

    cout << *max_element(codes.begin(), codes.end()) << "\n";

    return 0;
}
