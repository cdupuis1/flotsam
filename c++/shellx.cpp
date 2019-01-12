//
// Simple shell
//
// Simple user shell with more stuffs
//
#include <iostream>
#include <vector>
#include <sstream>
#include <fstream>
#include <cstring>
#include <unistd.h>
#include <sys/wait.h>

using namespace std;

const int word_size = 255;

class Shell {
private:
	string prompt;
	string config_file_name;

	void ChangeDir(vector<string> cmd);
	int HandleInternalCmd(vector<string> cmd);
public:
	Shell();
	void ParseConfigFile();
	void ExecuteLoop();
};

// Constructor
Shell::Shell()
{
	// Set default prompt
	prompt = "# ";

	// Set config file name
	config_file_name = ".sish";
}

void Shell::ParseConfigFile()
{
    string line;
    char *tmp;

    // Home directory for user is stored in HOME env variable
    tmp = getenv("HOME");
    if (!tmp) {
        cout << "Could not get HOME environmental variable\n" << endl;
        return;
    }
    
    // Attempt to open an input stream to the config file
    string config_file(tmp);
    config_file = config_file + "/" + config_file_name;
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

            // Set the prompt
            if (line_tokens[0] == "PROMPT")
                prompt = line_tokens[1];
        }
        myfile.close();
    }
}

void Shell::ChangeDir(vector<string> cmd)
{
	int rc = 0;
	rc = chdir(cmd[1].c_str());
	if (rc == -1 && errno == EFAULT )
		cout << "cd: Bad directory argument.\n";
	else
		perror("cd");
}

// Non-zero return value means we handled an internal shell command
int Shell::HandleInternalCmd(vector<string> cmd) {
	int rc = 0;
	if (cmd[0] == "exit") {
		exit(0);
	} else if (cmd[0] == "cd") {
		ChangeDir(cmd);
		rc = 1;
	}

	return rc;
}

// Main event loop
void Shell::ExecuteLoop() {
	pid_t pid, wait_pid;
	int status;
	string cmd;
	char **args = NULL;
	int num_args = 0;
	string token;
	int i = 0;
	vector<string> cmd_str_vec;
	int num_alloc = 0;

	while (1) {
		cout << prompt << " ";
		cmd.clear();
		getline(cin, cmd);

		// You shall not pass!
		if (cmd == "")
			continue;

		// System calls still require C style strings so we have to version the C++
		// to a istringstream so the string can be broken up and then we allocate a
		// a char * array to hold the program arguments
		istringstream cmd_token(cmd);
		num_args = 0;
		num_alloc = 0;
		cmd_str_vec.clear();

		// Break the string up by ' '
		while (getline(cmd_token, token, ' ')) {
			cmd_str_vec.push_back(token);
			num_args++;
		}

		// For an internal command we do not need to convert the string vecotr
		// to a C char array since we aren't passing anything to a system call
		if (HandleInternalCmd(cmd_str_vec))
			continue;

		// Allocate array of char + a terminating NULL
		num_alloc = num_args + 1;
		args = (char **) malloc(num_alloc * sizeof(char *));
		memset(args, 0, num_alloc * sizeof(char *));

		// Allocate space for strings
		for (i = 0; i < num_args; i++) {
			args[i] = (char *) malloc(word_size + 1);
			memset(args[i], 0, (word_size + 1));
		}

		// Copy the C++ string to array of C strings
		for (i = 0; i < num_args; i++) {
			// Bounds check to make sure we don't exceed the size of the C string
			int size = cmd_str_vec[i].size();
			if (size > word_size)
				size = word_size;
			strncpy(args[i], cmd_str_vec[i].c_str(), cmd_str_vec[i].size());
		}

		// NULL terminate the list
		args[num_args] == NULL;

		// Fork you!
		pid = fork();

		if (pid > 0) {
			// Shell waits for command to complete
			wait_pid = waitpid(pid, &status, 0);
		} else if (pid == 0) {
			// Child process
			status = execvp(args[0], args);
			if (status)
				perror("");
			// Make sure the child process exits here if exec fails
			exit(status);
		} else
			perror("fork");

		// Cleanup temporary C string allocations
		if (args) {
			for (i = 0; i < num_args; i++)
				if (args[i])
					free(args[i]);
			free(args);
		}
	}
}

int main() {
	Shell sh;

	sh.ParseConfigFile();
	sh.ExecuteLoop();

	return 0;
}
