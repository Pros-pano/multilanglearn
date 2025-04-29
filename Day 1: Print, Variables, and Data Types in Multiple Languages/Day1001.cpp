// C++ Day 1: Print, Variables, and Data Types

#include <iostream>     // Required for input/output operations
#include <string>       // Required for string operations
#include <limits>       // Required for checking min/max values of data types
#include <climits>      // Required for integer limits
#include <cfloat>       // Required for floating-point limits
#include <complex>      // Required for complex numbers
#include <vector>       // Required for vectors
#include <array>        // Required for arrays
#include <list>         // Required for linked lists
#include <map>          // Required for maps
#include <set>          // Required for sets
#include <queue>        // Required for queues
#include <stack>        // Required for stacks
#include <bitset>       // Required for bitsets
#include <memory>       // Required for smart pointers

int main() {
    // Print statements - displaying output to the console
    std::cout << "Hello, World!" << std::endl;  // Simple string output
    std::cout << "Welcome to C++ Programming!" << std::endl;
    
    // Variables - containers for storing data values
    std::string name = "Alice";  // String variable
    int age = 25;                // Integer variable
    double height = 5.7;         // Double variable
    bool isStudent = true;       // Boolean variable
    
    std::cout << "\nVariable demonstration:" << std::endl;
    std::cout << "Name: " << name << std::endl;
    std::cout << "Age: " << age << std::endl;
    std::cout << "Height: " << height << " feet" << std::endl;
    std::cout << "Is student? " << std::boolalpha << isStudent << std::endl;  // std::boolalpha shows true/false instead of 1/0
    
    // Multiple variable assignment
    int x = 10, y = 20, z = 30;
    std::cout << "\nMultiple variables: " << x << " " << y << " " << z << std::endl;
    
    // --------------------------------------------------------
    // C++ DATA TYPES - COMPREHENSIVE OVERVIEW
    // --------------------------------------------------------
    
    std::cout << "\n=== C++ DATA TYPES ===" << std::endl;
    
    // 1. INTEGER TYPES
    std::cout << "\n--- INTEGER TYPES ---" << std::endl;
    
    // Character (char) - typically 1 byte
    char character = 'A';
    std::cout << "\nChar: " << character << std::endl;
    std::cout << "Size of char: " << sizeof(char) << " bytes" << std::endl;
    std::cout << "Char as integer: " << static_cast<int>(character) << std::endl;
    std::cout << "Min char value: " << static_cast<int>(CHAR_MIN) << std::endl;
    std::cout << "Max char value: " << static_cast<int>(CHAR_MAX) << std::endl;
    
    // Signed char - guaranteed to be 1 byte
    signed char signedChar = -128;
    std::cout << "\nSigned char: " << static_cast<int>(signedChar) << std::endl;
    std::cout << "Size of signed char: " << sizeof(signed char) << " bytes" << std::endl;
    std::cout << "Min signed char value: " << static_cast<int>(SCHAR_MIN) << std::endl;
    std::cout << "Max signed char value: " << static_cast<int>(SCHAR_MAX) << std::endl;
    
    // Unsigned char - guaranteed to be 1 byte
    unsigned char unsignedChar = 255;
    std::cout << "\nUnsigned char: " << static_cast<int>(unsignedChar) << std::endl;
    std::cout << "Size of unsigned char: " << sizeof(unsigned char) << " bytes" << std::endl;
    std::cout << "Min unsigned char value: 0" << std::endl;
    std::cout << "Max unsigned char value: " << static_cast<int>(UCHAR_MAX) << std::endl;
    
    // Wide character (wchar_t)
    wchar_t wideChar = L'Ω';  // Unicode character
    std::cout << "\nWide character (as number): " << static_cast<int>(wideChar) << std::endl;
    std::cout << "Size of wchar_t: " << sizeof(wchar_t) << " bytes" << std::endl;
    std::wcout << L"Wide character: " << wideChar << std::endl;
    
    // Char16_t (C++11) - for UTF-16 characters
    char16_t char16 = u'Ω';
    std::cout << "\nChar16_t (as number): " << static_cast<int>(char16) << std::endl;
    std::cout << "Size of char16_t: " << sizeof(char16_t) << " bytes" << std::endl;
    
    // Char32_t (C++11) - for UTF-32 characters
    char32_t char32 = U'Ω';
    std::cout << "\nChar32_t (as number): " << static_cast<int>(char32) << std::endl;
    std::cout << "Size of char32_t: " << sizeof(char32_t) << " bytes" << std::endl;
    
    // Short int - typically 2 bytes
    short shortNum = 32767;
    std::cout << "\nShort int: " << shortNum << std::endl;
    std::cout << "Size of short: " << sizeof(short) << " bytes" << std::endl;
    std::cout << "Min short value: " << SHRT_MIN << std::endl;
    std::cout << "Max short value: " << SHRT_MAX << std::endl;
    
    // Unsigned short int
    unsigned short unsignedShort = 65535;
    std::cout << "\nUnsigned short: " << unsignedShort << std::endl;
    std::cout << "Size of unsigned short: " << sizeof(unsigned short) << " bytes" << std::endl;
    std::cout << "Min unsigned short value: 0" << std::endl;
    std::cout << "Max unsigned short value: " << USHRT_MAX << std::endl;
    
    // Integer (int) - typically 4 bytes
    int integerNum = 2147483647;
    std::cout << "\nInt: " << integerNum << std::endl;
    std::cout << "Size of int: " << sizeof(int) << " bytes" << std::endl;
    std::cout << "Min int value: " << INT_MIN << std::endl;
    std::cout << "Max int value: " << INT_MAX << std::endl;
    
    // Unsigned int
    unsigned int unsignedInt = 4294967295;
    std::cout << "\nUnsigned int: " << unsignedInt << std::endl;
    std::cout << "Size of unsigned int: " << sizeof(unsigned int) << " bytes" << std::endl;
    std::cout << "Min unsigned int value: 0" << std::endl;
    std::cout << "Max unsigned int value: " << UINT_MAX << std::endl;
    
    // Long int - at least 4 bytes
    long longNum = 2147483647L;
    std::cout << "\nLong int: " << longNum << std::endl;
    std::cout << "Size of long: " << sizeof(long) << " bytes" << std::endl;
    std::cout << "Min long value: " << LONG_MIN << std::endl;
    std::cout << "Max long value: " << LONG_MAX << std::endl;
    
    // Unsigned long int
    unsigned long unsignedLong = 4294967295UL;
    std::cout << "\nUnsigned long: " << unsignedLong << std::endl;
    std::cout << "Size of unsigned long: " << sizeof(unsigned long) << " bytes" << std::endl;
    std::cout << "Min unsigned long value: 0" << std::endl;
    std::cout << "Max unsigned long value: " << ULONG_MAX << std::endl;
    
    // Long long int (C++11) - at least 8 bytes
    long long longLongNum = 9223372036854775807LL;
    std::cout << "\nLong long int: " << longLongNum << std::endl;
    std::cout << "Size of long long: " << sizeof(long long) << " bytes" << std::endl;
    std::cout << "Min long long value: " << LLONG_MIN << std::endl;
    std::cout << "Max long long value: " << LLONG_MAX << std::endl;
    
    // Unsigned long long int (C++11)
    unsigned long long unsignedLongLong = 18446744073709551615ULL;
    std::cout << "\nUnsigned long long: " << unsignedLongLong << std::endl;
    std::cout << "Size of unsigned long long: " << sizeof(unsigned long long) << " bytes" << std::endl;
    std::cout << "Min unsigned long long value: 0" << std::endl;
    std::cout << "Max unsigned long long value: " << ULLONG_MAX << std::endl;
    
    // 2. FLOATING-POINT TYPES
    std::cout << "\n--- FLOATING-POINT TYPES ---" << std::endl;
    
    // Float - typically 4 bytes, 7 digits precision
    float floatNum = 3.14159f;
    std::cout << "\nFloat: " << floatNum << std::endl;
    std::cout << "Size of float: " << sizeof(float) << " bytes" << std::endl;
    std::cout << "Float digits of precision: " << FLT_DIG << std::endl;
    std::cout << "Min positive float value: " << FLT_MIN << std::endl;
    std::cout << "Max float value: " << FLT_MAX << std::endl;
    
    // Double - typically 8 bytes, 15 digits precision
    double doubleNum = 3.1415926535897932;
    std::cout << "\nDouble: " << doubleNum << std::endl;
    std::cout << "Size of double: " << sizeof(double) << " bytes" << std::endl;
    std::cout << "Double digits of precision: " << DBL_DIG << std::endl;
    std::cout << "Min positive double value: " << DBL_MIN << std::endl;
    std::cout << "Max double value: " << DBL_MAX << std::endl;
    
    // Long double - typically 12 or 16 bytes
    long double longDoubleNum = 3.1415926535897932384626433L;
    std::cout << "\nLong double: " << longDoubleNum << std::endl;
    std::cout << "Size of long double: " << sizeof(long double) << " bytes" << std::endl;
    std::cout << "Long double digits of precision: " << LDBL_DIG << std::endl;
    std::cout << "Min positive long double value: " << LDBL_MIN << std::endl;
    std::cout << "Max long double value: " << LDBL_MAX << std::endl;
    
    // 3. COMPLEX NUMBER TYPES
    std::cout << "\n--- COMPLEX NUMBER TYPES ---" << std::endl;
    
    // Complex float
    std::complex<float> complexFloat(3.0f, 4.0f);
    std::cout << "\nComplex float: " << complexFloat << std::endl;
    std::cout << "Size of complex<float>: " << sizeof(std::complex<float>) << " bytes" << std::endl;
    std::cout << "Real part: " << complexFloat.real() << std::endl;
    std::cout << "Imaginary part: " << complexFloat.imag() << std::endl;
    std::cout << "Absolute value (magnitude): " << std::abs(complexFloat) << std::endl;
    
    // Complex double
    std::complex<double> complexDouble(3.0, 4.0);
    std::cout << "\nComplex double: " << complexDouble << std::endl;
    std::cout << "Size of complex<double>: " << sizeof(std::complex<double>) << " bytes" << std::endl;
    
    // 4. BOOLEAN TYPE
    std::cout << "\n--- BOOLEAN TYPE ---" << std::endl;
    
    // Boolean (bool)
    bool boolTrue = true;
    bool boolFalse = false;
    std::cout << "\nBool (true): " << std::boolalpha << boolTrue << std::endl;
    std::cout << "Bool (false): " << boolFalse << std::endl;
    std::cout << "Size of bool: " << sizeof(bool) << " bytes" << std::endl;
    
    // 5. VOID TYPE
    std::cout << "\n--- VOID TYPE ---" << std::endl;
    std::cout << "Void is a special type that indicates 'no value'" << std::endl;
    std::cout << "It's commonly used for functions that don't return a value" << std::endl;
    
    // Void pointer - can point to any data type
    void* voidPtr = &age;
    std::cout << "\nVoid pointer address: " << voidPtr << std::endl;
    
    // 6. NULL POINTER
    std::cout << "\n--- NULL POINTER ---" << std::endl;
    
    // Null pointer (C-style)
    int* nullPtr = NULL;
    std::cout << "Null pointer (NULL): " << nullPtr << std::endl;
    
    // Null pointer (C++11)
    int* nullPtr2 = nullptr;
    std::cout << "Null pointer (nullptr): " << nullPtr2 << std::endl;
    
    // 7. STRING TYPES
    std::cout << "\n--- STRING TYPES ---" << std::endl;
    
    // C-style string (character array)
    char cString[] = "Hello C++";
    std::cout << "\nC-style string: " << cString << std::endl;
    std::cout << "Size: " << sizeof(cString) << " bytes (including null terminator)" << std::endl;
    
    // std::string - modern C++ string
    std::string stdString = "Hello modern C++";
    std::cout << "\nstd::string: " << stdString << std::endl;
    std::cout << "Length: " << stdString.length() << " characters" << std::endl;
    std::cout << "Capacity: " << stdString.capacity() << " characters" << std::endl;
    std::cout << "First character: " << stdString[0] << std::endl;
    std::cout << "Substring: " << stdString.substr(6, 6) << std::endl;
    
    // String concatenation
    std::string firstName = "John";
    std::string lastName = "Doe";
    std::string fullName = firstName + " " + lastName;
    std::cout << "\nConcatenated string: " << fullName << std::endl;
    
    // Wide string (wstring)
    std::wstring wideString = L"Wide string with Unicode: Ω";
    std::wcout << L"\nWide string (wstring): " << wideString << std::endl;
    
    // 8. CONTAINER TYPES FROM STL
    std::cout << "\n--- CONTAINER TYPES (STL) ---" << std::endl;
    
    // Array (fixed-size array)
    std::array<int, 5> stdArray = {1, 2, 3, 4, 5};
    std::cout << "\nstd::array: ";
    for (const auto& element : stdArray) {
        std::cout << element << " ";
    }
    std::cout << std::endl;
    std::cout << "Size: " << stdArray.size() << std::endl;
    std::cout << "First element: " << stdArray.front() << std::endl;
    std::cout << "Last element: " << stdArray.back() << std::endl;
    
    // Vector (dynamic array)
    std::vector<int> stdVector = {10, 20, 30, 40, 50};
    std::cout << "\nstd::vector: ";
    for (const auto& element : stdVector) {
        std::cout << element << " ";
    }
    std::cout << std::endl;
    std::cout << "Size: " << stdVector.size() << std::endl;
    std::cout << "Capacity: " << stdVector.capacity() << std::endl;
    stdVector.push_back(60);
    std::cout << "After push_back: ";
    for (const auto& element : stdVector) {
        std::cout << element << " ";
    }
    std::cout << std::endl;
    
    // List (doubly-linked list)
    std::list<int> stdList = {100, 200, 300};
    std::cout << "\nstd::list: ";
    for (const auto& element : stdList) {
        std::cout << element << " ";
    }
    std::cout << std::endl;
    stdList.push_front(50);
    stdList.push_back(400);
    std::cout << "After push operations: ";
    for (const auto& element : stdList) {
        std::cout << element << " ";
    }
    std::cout << std::endl;
    
    // Map (associative array)
    std::map<std::string, int> stdMap;
    stdMap["one"] = 1;
    stdMap["two"] = 2;
    stdMap["three"] = 3;
    std::cout << "\nstd::map: " << std::endl;
    for (const auto& pair : stdMap) {
        std::cout << pair.first << ": " << pair.second << std::endl;
    }
    std::cout << "Access by key (\"two\"): " << stdMap["two"] << std::endl;
    
    // Set (collection of unique values)
    std::set<int> stdSet = {10, 20, 30, 10, 20};  // Duplicates are ignored
    std::cout << "\nstd::set: ";
    for (const auto& element : stdSet) {
        std::cout << element << " ";
    }
    std::cout << std::endl;
    stdSet.insert(40);
    std::cout << "After insert: ";
    for (const auto& element : stdSet) {
        std::cout << element << " ";
    }
    std::cout << std::endl;

    // Queue (FIFO data structure)
    std::queue<int> stdQueue;
    stdQueue.push(100);
    stdQueue.push(200);
    stdQueue.push(300);
    std::cout << "\nstd::queue (front): " << stdQueue.front() << std::endl;
    std::cout << "std::queue (back): " << stdQueue.back() << std::endl;
    std::cout << "Size: " << stdQueue.size() << std::endl;
    stdQueue.pop();  // Removes the front element
    std::cout << "After pop, new front: " << stdQueue.front() << std::endl;
    
    // Stack (LIFO data structure)
    std::stack<int> stdStack;
    stdStack.push(100);
    stdStack.push(200);
    stdStack.push(300);
    std::cout << "\nstd::stack (top): " << stdStack.top() << std::endl;
    std::cout << "Size: " << stdStack.size() << std::endl;
    stdStack.pop();  // Removes the top element
    std::cout << "After pop, new top: " << stdStack.top() << std::endl;
    
    // 9. SPECIAL TYPES
    std::cout << "\n--- SPECIAL TYPES ---" << std::endl;
    
    // Enum (enumeration)
    enum Color { RED, GREEN, BLUE };  // By default, RED=0, GREEN=1, BLUE=2
    Color myColor = GREEN;
    std::cout << "\nEnum value: " << myColor << std::endl;
    
    // Enum class (C++11, strongly typed enumeration)
    enum class Direction { NORTH, EAST, SOUTH, WEST };
    Direction myDirection = Direction::EAST;
    std::cout << "Enum class used (cannot display raw value directly)" << std::endl;
    
    // Bitset
    std::bitset<8> bits(42);  // 00101010 in binary
    std::cout << "\nBitset (decimal 42): " << bits << std::endl;
    std::cout << "Bit at position 1: " << bits[1] << std::endl;
    std::cout << "Number of set bits: " << bits.count() << std::endl;
    bits.flip();  // Flip all bits
    std::cout << "After flip: " << bits << std::endl;
    
    // 10. SMART POINTERS (C++11)
    std::cout << "\n--- SMART POINTERS ---" << std::endl;
    
    // Unique pointer - exclusive ownership
    std::unique_ptr<int> uniquePtr = std::make_unique<int>(42);  // C++14
    std::cout << "\nUnique pointer value: " << *uniquePtr << std::endl;
    
    // Shared pointer - shared ownership
    std::shared_ptr<int> sharedPtr1 = std::make_shared<int>(100);
    std::shared_ptr<int> sharedPtr2 = sharedPtr1;  // Both point to the same memory
    std::cout << "\nShared pointer value: " << *sharedPtr1 << std::endl;
    std::cout << "Reference count: " << sharedPtr1.use_count() << std::endl;
    
    // Weak pointer - non-owning reference to a shared_ptr
    std::weak_ptr<int> weakPtr = sharedPtr1;
    std::cout << "\nWeak pointer created (doesn't affect reference count)" << std::endl;
    std::cout << "Reference count: " << sharedPtr1.use_count() << std::endl;
    
    // 11. TYPE INFERENCE (C++11)
    std::cout << "\n--- TYPE INFERENCE ---" << std::endl;
    
    // auto - automatically deduce variable type
    auto autoInt = 42;
    auto autoDouble = 3.14159;
    auto autoString = "auto string";
    
    std::cout << "\nAuto int: " << autoInt << std::endl;
    std::cout << "Auto double: " << autoDouble << std::endl;
    std::cout << "Auto string: " << autoString << std::endl;
    
    // decltype - get the type of an expression
    int num = 42;
    decltype(num) declInt = 100;  // Same type as num (int)
    decltype(3.14159) declDouble = 2.71828;  // double
    
    std::cout << "\nDecltype int: " << declInt << std::endl;
    std::cout << "Decltype double: " << declDouble << std::endl;
    
    // 12. TYPE CONVERSIONS
    std::cout << "\n--- TYPE CONVERSIONS ---" << std::endl;
    
    // Implicit conversion
    int intVal = 42;
    double doubleVal = intVal;  // int to double (no data loss)
    std::cout << "\nImplicit int to double: " << doubleVal << std::endl;
    
    // Potential data loss
    double piVal = 3.14159;
    int piInt = piVal;  // double to int (loss of fractional part)
    std::cout << "Double to int (data loss): " << piInt << std::endl;
    
    // C-style casting
    float floatVal = 2.71828f;
    int castInt = (int)floatVal;
    std::cout << "\nC-style cast (float to int): " << castInt << std::endl;
    
    // Modern C++ casting
    
    // static_cast - compile-time type checking
    double doubleVal2 = 3.14159;
    int staticCastInt = static_cast<int>(doubleVal2);
    std::cout << "\nstatic_cast (double to int): " << staticCastInt << std::endl;
    
    // reinterpret_cast - low-level reinterpretation of bit patterns
    int* intPtr = new int(42);
    long longFromPtr = reinterpret_cast<long>(intPtr);
    std::cout << "reinterpret_cast (pointer to long): " << longFromPtr << std::endl;
    int* ptrBack = reinterpret_cast<int*>(longFromPtr);
    std::cout << "reinterpret_cast (long back to pointer), value: " << *ptrBack << std::endl;
    
    // const_cast - add or remove const qualifier
    const int constVal = 100;
    int& nonConstRef = const_cast<int&>(constVal);  // Removing const (potentially dangerous)
    std::cout << "\nconst_cast (removing const): " << nonConstRef << std::endl;
    
    // dynamic_cast - safe downcast for polymorphic types
    // Used with inheritance, requires runtime type information
    
    // Memory cleanup
    delete intPtr;
    
    // 13. USER-DEFINED TYPES
    std::cout << "\n--- USER-DEFINED TYPES ---" << std::endl;
    
    // Struct - grouping related data
    struct Person {
        std::string name;
        int age;
        double height;
    };
    
    Person person1 = {"John", 30, 5.9};
    std::cout << "\nStruct Person: " << person1.name << ", " << person1.age << " years, " 
              << person1.height << " feet" << std::endl;
    
    // Union - memory-saving way to store different data types (one at a time)
    union Value {
        int intValue;
        double doubleValue;
        char charValue;
    };
    
    Value value;
    value.intValue = 42;
    std::cout << "\nUnion with int: " << value.intValue << std::endl;
    value.doubleValue = 3.14159;  // Overwrites the int value
    std::cout << "Union with double (now): " << value.doubleValue << std::endl;
    std::cout << "Union int value (corrupted): " << value.intValue << std::endl;
    
    // Class - the basis for OOP in C++
    std::cout << "\nClass (object-oriented programming):" << std::endl;
    std::cout << "A complex data type that encapsulates data and methods" << std::endl;
    
    return 0;
}