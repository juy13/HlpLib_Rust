// Dll_Test.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#include <iostream>
#include "my_header.h"

#pragma comment(lib, "userenv.lib")
#pragma comment(lib, "ws2_32.lib")   
 

int main()
{
    std::cout << add_numbers(2, 3) << std::endl;
    unsigned char arr[] = { 0x32, 0x45, 0xab, 0xFF, 0x00, 0x3C };
    size_t size = 0;
    int size2 = sizeof(arr);
    char out2[200] = { 0x00 };
   //unsigned char int = new char[200];
    ControlHex cntr;
    int rc = hex2ascii(size2, arr, &size, out2, cntr);
    //std::cout << (*out) << std::endl;
    for (int i = 0; i < size; i++)
    {
        std::cout << out2[i];
    }
    std::cout << std::endl;
}

