#!/usr/bin/python3
"""
This file generate the `./src/codes/tables.rs` file.
This is not a build.rs because it will mainly generated once
and I weill keep this script if we want to change how many bits
we consider.
To run just execute `$ python ./gen_code_tables.py`
"""
import os
from math import log2, ceil, floor
# Parameters
BITS_TO_CONSIDER = 8
MAX_VALUE = 2**BITS_TO_CONSIDER
MISS_VALUE = 255
ZETA_KS = [3]

def to_bin_padded(x: int) -> str:
    """Utility to get the binary representation of a value with the number of 
    zeros we actually care about."""
    # WARNING: this uses BITS_TO_CONSIDER which is a global
    return "0" * (BITS_TO_CONSIDER - len(bin(x)[2:])) + bin(x)[2:]

def lzcnt(x: int) -> int:
    """Utility that compute the number of leading zeros."""
    return (
       to_bin_padded(x) +'1'
    ).index('1')

# Compute the dst file path using the current file location as a repo-dir orcale
# so the script also works correctly if run outside of the folder
CURRENT_DIR = os.path.abspath(os.path.dirname(__file__))
TARGET_FILE = os.path.join(CURRENT_DIR, "src", "codes", "tables.rs")

# open the file
fd = open(TARGET_FILE, "w")

# Format string of how we are going to generate the tables of (value, bits)
fmt = """pub const {code_name}_TABLE: [(u8, u8); {size}] = [
{values}
];

"""
tuple_fmt = "\t({}, {}),    // {}"
################################################################################
# UNARY
################################################################################

def handle_unary(x: int):
    if x == 0:
        return MISS_VALUE, 0
    value = lzcnt(x)
    return value, value + 1

# Gen unary code
unary = "\n".join(
   tuple_fmt.format(*handle_unary(i), to_bin_padded(i))
    for i in range(MAX_VALUE)
)

fd.write(fmt.format(
    size=MAX_VALUE,
    values=unary,
    code_name="UNARY",
))

################################################################################
# Gamma
################################################################################

def handle_gamma(x: int):
    unary, unary_length = handle_unary(x)
    if unary_length == 0:
        return MISS_VALUE, 0

    code_length = 2 * unary_length - 1
    if code_length >= BITS_TO_CONSIDER:
        return MISS_VALUE, 0

    value = int(to_bin_padded(x)[unary + 1:code_length] or "0", 2)
    value += (1 << unary) - 1
    return value, code_length

# Gen gamma code
gamma = "\n".join(
   tuple_fmt.format(*handle_gamma(i), to_bin_padded(i))
    for i in range(MAX_VALUE)
)

fd.write(fmt.format(
    size=MAX_VALUE,
    values=gamma,
    code_name="GAMMA",
))

################################################################################
# Zeta
################################################################################

def handle_min_binary_m2l(bits: str, _max: int):
    u = ceil(log2(_max))
    l = floor(log2(_max))

    if len(bits) < l:
        return MISS_VALUE, 0

    n = int(bits[:l] or "0", 2)
    
    scarto = 2**u - _max
    if n < scarto:
        return n, l
    
    if len(bits) < u:
        return MISS_VALUE, 0

    return int(bits[:u] or "0", 2) - scarto, u

def handle_zeta_m2l(x: int, K: int):
    unary, unary_length = handle_unary(x)
    if unary_length == 0:
        return MISS_VALUE, 0

    h = unary
    u = 2**((h + 1) * K)
    l = 2**(h * K)

    # Decode the binary
    bits = to_bin_padded(x)[unary_length:]
    r, min_binary_length = handle_min_binary_m2l(bits, u - l)

    if min_binary_length == 0:
        return MISS_VALUE, 0
    
    return l + r - 1, unary_length + min_binary_length

# Gen zeta code for all the K needed
for K in ZETA_KS:
    zeta = "\n".join(
    tuple_fmt.format(*handle_zeta_m2l(i, K), to_bin_padded(i))
        for i in range(MAX_VALUE)
    )

    fd.write(fmt.format(
        size=MAX_VALUE,
        values=zeta,
        code_name="ZETA%d_M2L"%K,
    ))