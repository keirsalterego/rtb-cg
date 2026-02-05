#include <iostream>

extern "C" {
    void tpde_initialize_session() {
        std::cout << "[tpde-native] session started. initializing codegen context..." << std::endl;
    }

    void tpde_emit_phi_node(const char* dest_reg, const char** blocks, int* values, int count) {
        std::cout << "[tpde-native] building phi node for " << dest_reg << " with " << count << " edges" << std::endl;
        for (int i = 0; i < count; ++i) {
            std::cout << "  <- block: " << blocks[i] << " | value: " << values[i] << std::endl;
        }
    }
}