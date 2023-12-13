#include <iostream>
#include <variant>
#include <string>
#include <vector>

struct AliasTo {
    uint32_t index;
    uint32_t target;
};

struct Zone {
    uint32_t index;
    std::vector<int32_t> transPre32 ;
    std::vector<int32_t> trans;
    std::vector<int32_t> transPost32;
    std::vector<int64_t> typeOffsets;
    std::vector<uint8_t> typeMap;
    const char *const finalRule;
    int32_t finalRaw;
    int32_t finalYear;
    std::vector<uint16_t> aliases; 
};

struct Rule {
    const char *const id;
    int32_t values[11];
};

const Zone kZonesDetails[] = {
#include "zones.def"
};

const AliasTo kZoneAliases[] = {
#include "aliases.def"
};

const Rule kRules[] = {
#include "rules.def"
};

const char* names[] = {
#include "names.def"
};

const char* regions[] = {
#include "regions.def"
};

int main() {
    std::cout << sizeof(kZonesDetails) << ", " << sizeof(kZonesDetails) / sizeof(kZonesDetails[0]) << "\n";
    return 0;
}