// Dll_Test.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#include <iostream>
#include "my_header.h"

#pragma comment(lib, "userenv.lib")
#pragma comment(lib, "ws2_32.lib")   

int a2h(char* ascii_ptr, unsigned char* hex_ptr, int len);

int main()
{
    std::cout << add_numbers(2, 3) << std::endl;
    unsigned char arr[] = { 0x32, 0x45, 0xab, 0xFF, 0x00, 0x3C };
    unsigned char out1[200] = { 0x00 };
    size_t size = 200;
    int size2 = sizeof(arr);
    char out2[200] = { 0x00 };
   //unsigned char int = new char[200];
    ControlHex cntr;
    int rc = hex2ascii(6, arr, &size, out2, WithOut_Space);
    //std::cout << (*out) << std::endl;
    for (int i = 0; i < size; i++)
    {
        std::cout << out2[i];
    }
    std::cout << std::endl;


    ascii2hex(size * 2, out2, &size, arr, WithOut_Space);

}

//int a2h(char* ascii_ptr, unsigned char* hex_ptr, int len)
//{
//    for (int i = 0; i < (len / 2); i++)
//    {
//
//        *(hex_ptr + i) = (*(ascii_ptr + (2 * i)) <= '9') ? ((*(ascii_ptr + (2 * i)) - '0') * 16) : (((*(ascii_ptr + (2 * i)) - 'A') + 10) << 4);
//        *(hex_ptr + i) |= (*(ascii_ptr + (2 * i) + 1) <= '9') ? (*(ascii_ptr + (2 * i) + 1) - '0') : (*(ascii_ptr + (2 * i) + 1) - 'A' + 10);
//
//    }
//    return 0;
//}
