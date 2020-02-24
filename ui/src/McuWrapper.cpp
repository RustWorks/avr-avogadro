#include "McuWrapper.h"
#include <vector>

extern "C" {
    void mcu_step(void* mcu);
    void mcu_get_register_array(void* mcu, const unsigned char* buffer);
    void mcu_set_register_array(void* mcu, const unsigned char* buffer);
    void mcu_set_register(void* mcu, char registerId, char value);
    short mcu_get_program_counter(void* mcu);
    void mcu_set_program_counter(void* mcu, short value);
    short mcu_get_stack_pointer(void* mcu);
    short mcu_get_current_instruction(void* mcu);
    void mcu_display_current_instruction(void* mcu, const char* buffer, size_t size);
    void mcu_load_file(void* mcu, const char* filename);
    size_t mcu_get_memory_size(void* mcu);
    void mcu_get_memory_data(void* mcu, const char* buffer, size_t size);
    unsigned char mcu_get_flags(void* mcu);
    void mcu_set_flags(void* mcu, unsigned char flags);
}

McuWrapper::McuWrapper(void* mcu) : mcu(mcu) {}

void McuWrapper::step() const {
    mcu_step(this->mcu);
}

void McuWrapper::getRegisterArray(const unsigned char* buffer) const {
    mcu_get_register_array(this->mcu, buffer);
}

void McuWrapper::setRegisterArray(const unsigned char* buffer) const {
    mcu_set_register_array(this->mcu, buffer);
}

void McuWrapper::setRegister(char registerId, char value) const {
    mcu_set_register(this->mcu, registerId, value);
}

short McuWrapper::getProgramCounter() const {
    return mcu_get_program_counter(this->mcu);
}

void McuWrapper::setProgramCounter(short value) const {
    mcu_set_program_counter(this->mcu, value);
}

short McuWrapper::getStackPointer() const {
    return mcu_get_stack_pointer(this->mcu);
}

short McuWrapper::getCurrentInstruction() const {
    return mcu_get_current_instruction(this->mcu);
}

void McuWrapper::displayCurrentInstruction(const char* buffer, std::size_t size) const {
    mcu_display_current_instruction(this->mcu, buffer, size);
}

void McuWrapper::getMemoryBank(std::vector<char>& buffer) const {
    size_t buf_size = mcu_get_memory_size(this->mcu);
    buffer.resize(buf_size);
    mcu_get_memory_data(this->mcu, buffer.data(), buf_size);
}

unsigned char McuWrapper::getFlags() const {
    return mcu_get_flags(this->mcu);
}

void McuWrapper::loadFile(const char* filename) const {
    mcu_load_file(this->mcu, filename);
}