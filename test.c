
typedef unsigned long long int u64;

const u64 VALUE_SIZE = 5;
const u64 WORD_SIZE = 64;
const u64 VALUE_MASK = 0b11111;

void write(u64 * array, u64 index, u64 value, u64 value_size) {
    u64 pos = value_size * index;
    u64 base = pos / WORD_SIZE;
    u64 o1 = pos % WORD_SIZE;
    u64 lower = value << o1;
    
    u64 o2 = WORD_SIZE - o1;
    u64 higher = value >> o2;

    array[base] |= lower;
    array[base + 1] |= higher;
}

u64 read(u64 * array, u64 index, u64 value_size) {
    u64 pos = value_size * index;
    u64 base = pos / WORD_SIZE;
    u64 o1 = pos % WORD_SIZE;
    u64 o2 = WORD_SIZE - o1;

    u64 lower  = (array[base] >> o1) & VALUE_MASK;
    u64 higher  = (array[base + 1] << o2) & VALUE_MASK;
    return (higher | lower) & VALUE_MASK;
}

/*
clang 10.0.1 -O3 -march=native
write:                                  # @write
        imul    rsi, rcx
        mov     rax, rsi
        shr     rax, 6
        shlx    rcx, rdx, rsi
        and     sil, 63
        neg     sil
        shrx    rdx, rdx, rsi
        or      qword ptr [rdi + 8*rax], rcx
        or      qword ptr [rdi + 8*rax + 8], rdx
        ret
read:                                   # @read
        imul    rsi, rdx
        mov     rax, rsi
        shr     rax, 6
        shrx    rcx, qword ptr [rdi + 8*rax], rsi
        and     sil, 63
        neg     sil
        shlx    rax, qword ptr [rdi + 8*rax + 8], rsi
        or      eax, ecx
        and     eax, 31
        ret
VALUE_SIZE:
        .quad   5                       # 0x5

WORD_SIZE:
        .quad   64                      # 0x40

VALUE_MASK:
        .quad   31                      # 0x1f
*/