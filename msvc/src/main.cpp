extern "C"
{
	int add(int, int);
	char* get_string();
	void free_string(char*);
}

#pragma comment(lib,"ws2_32.lib")
#pragma comment(lib,"rust.lib")
#pragma comment(lib,"userenv.lib")

#include <iostream>
#include <string>

int main(int argc, char* argv[])
{
	std::cout << "hello world" << std::endl;

	int a = 3;
	int b = 4;
	int x = add(a, b);
	std::cout << a << " + " << b << " = " << x << std::endl;

	char* y = get_string();
	std::string str(y);
	std::cout << str << std::endl;
	free_string(y);

	std::cin.get();
	return 0;
}