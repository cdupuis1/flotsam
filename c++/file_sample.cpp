//
// file_sample.cpp
//
// Sample to open a file and look for a config variable.
//
#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>

using namespace std;

int main ()
{
    string line;
    char *tmp;

    // Home directory for user is stored in HOME env variable
    tmp = getenv("HOME");
    if (!tmp) {
        cout << "Could not get HOME environmental variable\n" << endl;
        exit(1);
    }
    
    // Attempt to open an input stream to the config file
    string config_file(tmp);
    config_file = config_file + "/.sish";
    ifstream myfile(config_file);
    
    if (myfile.is_open())
    {
        while (getline (myfile,line))
        {
            istringstream line_token(line);
            vector<string> line_tokens;
            string token;

            // Use getline to tokenize the line from the config file
            while (getline(line_token, token, '=')) {
                line_tokens.push_back(token);
            }

            // For the purposes of the sample we'll just look for prompt
            if (line_tokens[0] == "PROMPT")
                cout << "prompt=" << line_tokens[1] << endl;
        }
        myfile.close();
    } else {
        cout << "Couldn't open " << config_file << endl;
    }
}