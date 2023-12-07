#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <algorithm>
using namespace std;

int main() {
    ifstream file("input2");
    vector<std::string> lines;
    vector<std::string> output;
    vector<std::string> output2;
    string line;
    int num = 0;
    int index = 0;


    while (std::getline(file, line)) {
        lines.push_back(line);
    }

    for (auto& line : lines) {
        for (size_t x = 0; x + 5 <= line.length(); x++) {
            string seq = line.substr(x, 5);


            size_t found = seq.find("three");
            size_t lfound = line.find(seq);
            while (found != std::string::npos) {
                
                seq.replace(found +1 , 1, "3");
                found = seq.find("three", found + 1); // Search for the next occurrence
                line.replace(lfound, 5, seq);
            }   

            found = seq.find("seven");
            while (found != std::string::npos) {
                
                seq.replace(found +1 , 1, "7");
                found = seq.find("seven", found + 1); // Search for the next occurrence
                line.replace(lfound, 5, seq);
            }
            found = seq.find("eight");
            while (found != std::string::npos) {
                
                seq.replace(found +1 , 1, "8");
                found = seq.find("eight", found + 1); // Search for the next occurrence
                line.replace(lfound, 5, seq);
            }


            found = seq.find("four");
            while (found != std::string::npos) {
                
                seq.replace(found +1 , 1, "4");
                found = seq.find("four", found + 1); // Search for the next occurrence
                line.replace(lfound, 5, seq);
            }
            found = seq.find("five");
            while (found != std::string::npos) {
                seq.replace(found +1 , 1, "5");
                found = seq.find("five", found + 1); // Search for the next occurrence
                line.replace(lfound, 5, seq);
            }
            found = seq.find("nine");
            while (found != std::string::npos) {
                
                seq.replace(found +1 , 1, "9");
                found = seq.find("nine", found + 1); // Search for the next occurrence
                line.replace(lfound, 5, seq);
            }
            found = seq.find("six");
            while (found != std::string::npos) {
                
                seq.replace(found +1 , 1, "6");
                found = seq.find("six", found + 1); // Search for the next occurrence
                line.replace(lfound, 5, seq);
            }


            found = seq.find("one");
            while (found != std::string::npos) {
                
                seq.replace(found +1 , 1, "1");
                found = seq.find("one", found + 1); // Search for the next occurrence
                line.replace(lfound, 5, seq);
            }
            found = seq.find("two");
            while (found != std::string::npos) {
                
                seq.replace(found +1 , 1, "2");
                found = seq.find("two", found + 1); // Search for the next occurrence
                line.replace(lfound, 5, seq);
            }

        }


        // cout << line << endl;
        for (char c : line) {
                if(c >= '0' && c <= '9') {

                    output.push_back(to_string(c));
                    break;
                }
        }

        std::reverse(line.begin(), line.end());

        for (char c : line) {
                if(c >= '0' && c <= '9') {

                    output2.push_back(to_string(c));
                    break;
                }
        }

    }

    while (index < output.size()) {

        num = num + stoi(to_string((stoi(output[index])-48)) + to_string((stoi(output2[index])-48)));
        index++;
    }
    cout << num << endl;

    return 0;
}