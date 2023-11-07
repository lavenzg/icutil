#include <iostream>
#include <variant>
#include <string>
#include <vector>

struct AliasTo {
    uint32_t target;
};

struct Zone {
    uint32_t index;
    std::vector<int32_t> transPre32 ;
    std::vector<int32_t> trans;
    std::vector<int32_t> transPost32;
    std::vector<int64_t> typeOffsets;
    std::vector<uint8_t> typeMap;
    std::string finalRule;
    int32_t finalRaw;
    int32_t finalYear;
    std::vector<uint16_t> aliases; 
};

const Zone kTzData[] = {
    {0, {1, 2, 3}, {1, 2}, {}, {1, 2}, {0, 1}, "tz", 12, 12, {0, 1}}
};

int main() {
    std::cout << sizeof(kTzData) << ", " << sizeof(kTzData) / sizeof(kTzData[0]) << "\n";
    return 0;
}