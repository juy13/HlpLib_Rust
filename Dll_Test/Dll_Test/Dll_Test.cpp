// Dll_Test.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#include <iostream>
#include "my_header.h"

#pragma comment(lib, "userenv.lib")
#pragma comment(lib, "ws2_32.lib")   

//int a2h(char* ascii_ptr, unsigned char* hex_ptr, int len);

int main()
{
    std::cout << add_numbers(2, 3) << std::endl;
    unsigned char arr[] = { 0x32, 0x45, 0xab, 0xFF, 0x00, 0x3C };
    unsigned char out1[4000] = { 0x00 };
    size_t size = 400;
    int size2 = sizeof(arr);
    char out2[200] = "57 49 41 33 25 17 09 01 58 50 42 34 26 18 10 02 59 51 43 35 27 19 11 03 60 52 44 36 63 55 47 39 31 23 15 07 62 54 46 38 30 22 14 06 61 53 45 37 29 21 13 05 28 20 12 04";
   //unsigned char int = new char[200];
    ControlHex cntr;
    //int rc = hex2ascii(6, arr, &size, out2, WithOut_Space);
    //std::cout << (*out) << std::endl;
    //for (int i = 0; i < size; i++)
    //{
    //    std::cout << out2[i];
    //}
    //std::cout << std::endl;

    size2 = strlen(out2);
    //size = 200;

    std::cout << "Ans: " << ascii2hex(size2, out2, &size, out1, With_Space) << std::endl;

    std::cout << memcmp(out1, arr, size) << std::endl;

}

