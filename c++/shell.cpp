//
// Simple shell
//
// Simple user shell written in C++
//
#include <iostream>
#include <vector>
#include <sstream>
#include <cstring>
#include <unistd.h>
#include <sys/wait.h>

using namespace std;

const int word_size = 255;

class Shell {
public:
	void ChangeDir(vector<string> cmd);
	int HandleInternalCmd(vector<string> cmd);
public:
	void ExecuteLoop();
};

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

void Shell::ExecuteLoop() {
	pid_t pid, wait_pid;
	int status;
	string cmd;
	char **args = NULL;
	int num_args = 0;
	string token;
	int i = 0;
	vector<string> cmd_str_vec;

	while (1) {
		cout << "# ";
		cmd.clear();
		getline(cin, cmd);

		// System calls still require C style strings so we have to version the C++
		// to a istringstream so the string can be broken up and then we allocate a
		// a char * array to hold the program arguments
		istringstream cmd_token(cmd);
		num_args = 0;
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

		// Allocate array of char
		args = (char **) malloc(num_args * sizeof(char *));
		memset(args, 0, num_args * sizeof(char *));

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

	sh.ExecuteLoop();

	return 0;
}
