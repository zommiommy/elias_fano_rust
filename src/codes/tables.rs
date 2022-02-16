pub const UNARY_TABLE: [(u8, u8); 256] = [
	(255, 0),    // 00000000
	(7, 8),    // 00000001
	(6, 7),    // 00000010
	(6, 7),    // 00000011
	(5, 6),    // 00000100
	(5, 6),    // 00000101
	(5, 6),    // 00000110
	(5, 6),    // 00000111
	(4, 5),    // 00001000
	(4, 5),    // 00001001
	(4, 5),    // 00001010
	(4, 5),    // 00001011
	(4, 5),    // 00001100
	(4, 5),    // 00001101
	(4, 5),    // 00001110
	(4, 5),    // 00001111
	(3, 4),    // 00010000
	(3, 4),    // 00010001
	(3, 4),    // 00010010
	(3, 4),    // 00010011
	(3, 4),    // 00010100
	(3, 4),    // 00010101
	(3, 4),    // 00010110
	(3, 4),    // 00010111
	(3, 4),    // 00011000
	(3, 4),    // 00011001
	(3, 4),    // 00011010
	(3, 4),    // 00011011
	(3, 4),    // 00011100
	(3, 4),    // 00011101
	(3, 4),    // 00011110
	(3, 4),    // 00011111
	(2, 3),    // 00100000
	(2, 3),    // 00100001
	(2, 3),    // 00100010
	(2, 3),    // 00100011
	(2, 3),    // 00100100
	(2, 3),    // 00100101
	(2, 3),    // 00100110
	(2, 3),    // 00100111
	(2, 3),    // 00101000
	(2, 3),    // 00101001
	(2, 3),    // 00101010
	(2, 3),    // 00101011
	(2, 3),    // 00101100
	(2, 3),    // 00101101
	(2, 3),    // 00101110
	(2, 3),    // 00101111
	(2, 3),    // 00110000
	(2, 3),    // 00110001
	(2, 3),    // 00110010
	(2, 3),    // 00110011
	(2, 3),    // 00110100
	(2, 3),    // 00110101
	(2, 3),    // 00110110
	(2, 3),    // 00110111
	(2, 3),    // 00111000
	(2, 3),    // 00111001
	(2, 3),    // 00111010
	(2, 3),    // 00111011
	(2, 3),    // 00111100
	(2, 3),    // 00111101
	(2, 3),    // 00111110
	(2, 3),    // 00111111
	(1, 2),    // 01000000
	(1, 2),    // 01000001
	(1, 2),    // 01000010
	(1, 2),    // 01000011
	(1, 2),    // 01000100
	(1, 2),    // 01000101
	(1, 2),    // 01000110
	(1, 2),    // 01000111
	(1, 2),    // 01001000
	(1, 2),    // 01001001
	(1, 2),    // 01001010
	(1, 2),    // 01001011
	(1, 2),    // 01001100
	(1, 2),    // 01001101
	(1, 2),    // 01001110
	(1, 2),    // 01001111
	(1, 2),    // 01010000
	(1, 2),    // 01010001
	(1, 2),    // 01010010
	(1, 2),    // 01010011
	(1, 2),    // 01010100
	(1, 2),    // 01010101
	(1, 2),    // 01010110
	(1, 2),    // 01010111
	(1, 2),    // 01011000
	(1, 2),    // 01011001
	(1, 2),    // 01011010
	(1, 2),    // 01011011
	(1, 2),    // 01011100
	(1, 2),    // 01011101
	(1, 2),    // 01011110
	(1, 2),    // 01011111
	(1, 2),    // 01100000
	(1, 2),    // 01100001
	(1, 2),    // 01100010
	(1, 2),    // 01100011
	(1, 2),    // 01100100
	(1, 2),    // 01100101
	(1, 2),    // 01100110
	(1, 2),    // 01100111
	(1, 2),    // 01101000
	(1, 2),    // 01101001
	(1, 2),    // 01101010
	(1, 2),    // 01101011
	(1, 2),    // 01101100
	(1, 2),    // 01101101
	(1, 2),    // 01101110
	(1, 2),    // 01101111
	(1, 2),    // 01110000
	(1, 2),    // 01110001
	(1, 2),    // 01110010
	(1, 2),    // 01110011
	(1, 2),    // 01110100
	(1, 2),    // 01110101
	(1, 2),    // 01110110
	(1, 2),    // 01110111
	(1, 2),    // 01111000
	(1, 2),    // 01111001
	(1, 2),    // 01111010
	(1, 2),    // 01111011
	(1, 2),    // 01111100
	(1, 2),    // 01111101
	(1, 2),    // 01111110
	(1, 2),    // 01111111
	(0, 1),    // 10000000
	(0, 1),    // 10000001
	(0, 1),    // 10000010
	(0, 1),    // 10000011
	(0, 1),    // 10000100
	(0, 1),    // 10000101
	(0, 1),    // 10000110
	(0, 1),    // 10000111
	(0, 1),    // 10001000
	(0, 1),    // 10001001
	(0, 1),    // 10001010
	(0, 1),    // 10001011
	(0, 1),    // 10001100
	(0, 1),    // 10001101
	(0, 1),    // 10001110
	(0, 1),    // 10001111
	(0, 1),    // 10010000
	(0, 1),    // 10010001
	(0, 1),    // 10010010
	(0, 1),    // 10010011
	(0, 1),    // 10010100
	(0, 1),    // 10010101
	(0, 1),    // 10010110
	(0, 1),    // 10010111
	(0, 1),    // 10011000
	(0, 1),    // 10011001
	(0, 1),    // 10011010
	(0, 1),    // 10011011
	(0, 1),    // 10011100
	(0, 1),    // 10011101
	(0, 1),    // 10011110
	(0, 1),    // 10011111
	(0, 1),    // 10100000
	(0, 1),    // 10100001
	(0, 1),    // 10100010
	(0, 1),    // 10100011
	(0, 1),    // 10100100
	(0, 1),    // 10100101
	(0, 1),    // 10100110
	(0, 1),    // 10100111
	(0, 1),    // 10101000
	(0, 1),    // 10101001
	(0, 1),    // 10101010
	(0, 1),    // 10101011
	(0, 1),    // 10101100
	(0, 1),    // 10101101
	(0, 1),    // 10101110
	(0, 1),    // 10101111
	(0, 1),    // 10110000
	(0, 1),    // 10110001
	(0, 1),    // 10110010
	(0, 1),    // 10110011
	(0, 1),    // 10110100
	(0, 1),    // 10110101
	(0, 1),    // 10110110
	(0, 1),    // 10110111
	(0, 1),    // 10111000
	(0, 1),    // 10111001
	(0, 1),    // 10111010
	(0, 1),    // 10111011
	(0, 1),    // 10111100
	(0, 1),    // 10111101
	(0, 1),    // 10111110
	(0, 1),    // 10111111
	(0, 1),    // 11000000
	(0, 1),    // 11000001
	(0, 1),    // 11000010
	(0, 1),    // 11000011
	(0, 1),    // 11000100
	(0, 1),    // 11000101
	(0, 1),    // 11000110
	(0, 1),    // 11000111
	(0, 1),    // 11001000
	(0, 1),    // 11001001
	(0, 1),    // 11001010
	(0, 1),    // 11001011
	(0, 1),    // 11001100
	(0, 1),    // 11001101
	(0, 1),    // 11001110
	(0, 1),    // 11001111
	(0, 1),    // 11010000
	(0, 1),    // 11010001
	(0, 1),    // 11010010
	(0, 1),    // 11010011
	(0, 1),    // 11010100
	(0, 1),    // 11010101
	(0, 1),    // 11010110
	(0, 1),    // 11010111
	(0, 1),    // 11011000
	(0, 1),    // 11011001
	(0, 1),    // 11011010
	(0, 1),    // 11011011
	(0, 1),    // 11011100
	(0, 1),    // 11011101
	(0, 1),    // 11011110
	(0, 1),    // 11011111
	(0, 1),    // 11100000
	(0, 1),    // 11100001
	(0, 1),    // 11100010
	(0, 1),    // 11100011
	(0, 1),    // 11100100
	(0, 1),    // 11100101
	(0, 1),    // 11100110
	(0, 1),    // 11100111
	(0, 1),    // 11101000
	(0, 1),    // 11101001
	(0, 1),    // 11101010
	(0, 1),    // 11101011
	(0, 1),    // 11101100
	(0, 1),    // 11101101
	(0, 1),    // 11101110
	(0, 1),    // 11101111
	(0, 1),    // 11110000
	(0, 1),    // 11110001
	(0, 1),    // 11110010
	(0, 1),    // 11110011
	(0, 1),    // 11110100
	(0, 1),    // 11110101
	(0, 1),    // 11110110
	(0, 1),    // 11110111
	(0, 1),    // 11111000
	(0, 1),    // 11111001
	(0, 1),    // 11111010
	(0, 1),    // 11111011
	(0, 1),    // 11111100
	(0, 1),    // 11111101
	(0, 1),    // 11111110
	(0, 1),    // 11111111
];

pub const GAMMA_TABLE: [(u8, u8); 256] = [
	(255, 0),    // 00000000
	(255, 0),    // 00000001
	(255, 0),    // 00000010
	(255, 0),    // 00000011
	(255, 0),    // 00000100
	(255, 0),    // 00000101
	(255, 0),    // 00000110
	(255, 0),    // 00000111
	(255, 0),    // 00001000
	(255, 0),    // 00001001
	(255, 0),    // 00001010
	(255, 0),    // 00001011
	(255, 0),    // 00001100
	(255, 0),    // 00001101
	(255, 0),    // 00001110
	(255, 0),    // 00001111
	(7, 7),    // 00010000
	(7, 7),    // 00010001
	(8, 7),    // 00010010
	(8, 7),    // 00010011
	(9, 7),    // 00010100
	(9, 7),    // 00010101
	(10, 7),    // 00010110
	(10, 7),    // 00010111
	(11, 7),    // 00011000
	(11, 7),    // 00011001
	(12, 7),    // 00011010
	(12, 7),    // 00011011
	(13, 7),    // 00011100
	(13, 7),    // 00011101
	(14, 7),    // 00011110
	(14, 7),    // 00011111
	(3, 5),    // 00100000
	(3, 5),    // 00100001
	(3, 5),    // 00100010
	(3, 5),    // 00100011
	(3, 5),    // 00100100
	(3, 5),    // 00100101
	(3, 5),    // 00100110
	(3, 5),    // 00100111
	(4, 5),    // 00101000
	(4, 5),    // 00101001
	(4, 5),    // 00101010
	(4, 5),    // 00101011
	(4, 5),    // 00101100
	(4, 5),    // 00101101
	(4, 5),    // 00101110
	(4, 5),    // 00101111
	(5, 5),    // 00110000
	(5, 5),    // 00110001
	(5, 5),    // 00110010
	(5, 5),    // 00110011
	(5, 5),    // 00110100
	(5, 5),    // 00110101
	(5, 5),    // 00110110
	(5, 5),    // 00110111
	(6, 5),    // 00111000
	(6, 5),    // 00111001
	(6, 5),    // 00111010
	(6, 5),    // 00111011
	(6, 5),    // 00111100
	(6, 5),    // 00111101
	(6, 5),    // 00111110
	(6, 5),    // 00111111
	(1, 3),    // 01000000
	(1, 3),    // 01000001
	(1, 3),    // 01000010
	(1, 3),    // 01000011
	(1, 3),    // 01000100
	(1, 3),    // 01000101
	(1, 3),    // 01000110
	(1, 3),    // 01000111
	(1, 3),    // 01001000
	(1, 3),    // 01001001
	(1, 3),    // 01001010
	(1, 3),    // 01001011
	(1, 3),    // 01001100
	(1, 3),    // 01001101
	(1, 3),    // 01001110
	(1, 3),    // 01001111
	(1, 3),    // 01010000
	(1, 3),    // 01010001
	(1, 3),    // 01010010
	(1, 3),    // 01010011
	(1, 3),    // 01010100
	(1, 3),    // 01010101
	(1, 3),    // 01010110
	(1, 3),    // 01010111
	(1, 3),    // 01011000
	(1, 3),    // 01011001
	(1, 3),    // 01011010
	(1, 3),    // 01011011
	(1, 3),    // 01011100
	(1, 3),    // 01011101
	(1, 3),    // 01011110
	(1, 3),    // 01011111
	(2, 3),    // 01100000
	(2, 3),    // 01100001
	(2, 3),    // 01100010
	(2, 3),    // 01100011
	(2, 3),    // 01100100
	(2, 3),    // 01100101
	(2, 3),    // 01100110
	(2, 3),    // 01100111
	(2, 3),    // 01101000
	(2, 3),    // 01101001
	(2, 3),    // 01101010
	(2, 3),    // 01101011
	(2, 3),    // 01101100
	(2, 3),    // 01101101
	(2, 3),    // 01101110
	(2, 3),    // 01101111
	(2, 3),    // 01110000
	(2, 3),    // 01110001
	(2, 3),    // 01110010
	(2, 3),    // 01110011
	(2, 3),    // 01110100
	(2, 3),    // 01110101
	(2, 3),    // 01110110
	(2, 3),    // 01110111
	(2, 3),    // 01111000
	(2, 3),    // 01111001
	(2, 3),    // 01111010
	(2, 3),    // 01111011
	(2, 3),    // 01111100
	(2, 3),    // 01111101
	(2, 3),    // 01111110
	(2, 3),    // 01111111
	(0, 1),    // 10000000
	(0, 1),    // 10000001
	(0, 1),    // 10000010
	(0, 1),    // 10000011
	(0, 1),    // 10000100
	(0, 1),    // 10000101
	(0, 1),    // 10000110
	(0, 1),    // 10000111
	(0, 1),    // 10001000
	(0, 1),    // 10001001
	(0, 1),    // 10001010
	(0, 1),    // 10001011
	(0, 1),    // 10001100
	(0, 1),    // 10001101
	(0, 1),    // 10001110
	(0, 1),    // 10001111
	(0, 1),    // 10010000
	(0, 1),    // 10010001
	(0, 1),    // 10010010
	(0, 1),    // 10010011
	(0, 1),    // 10010100
	(0, 1),    // 10010101
	(0, 1),    // 10010110
	(0, 1),    // 10010111
	(0, 1),    // 10011000
	(0, 1),    // 10011001
	(0, 1),    // 10011010
	(0, 1),    // 10011011
	(0, 1),    // 10011100
	(0, 1),    // 10011101
	(0, 1),    // 10011110
	(0, 1),    // 10011111
	(0, 1),    // 10100000
	(0, 1),    // 10100001
	(0, 1),    // 10100010
	(0, 1),    // 10100011
	(0, 1),    // 10100100
	(0, 1),    // 10100101
	(0, 1),    // 10100110
	(0, 1),    // 10100111
	(0, 1),    // 10101000
	(0, 1),    // 10101001
	(0, 1),    // 10101010
	(0, 1),    // 10101011
	(0, 1),    // 10101100
	(0, 1),    // 10101101
	(0, 1),    // 10101110
	(0, 1),    // 10101111
	(0, 1),    // 10110000
	(0, 1),    // 10110001
	(0, 1),    // 10110010
	(0, 1),    // 10110011
	(0, 1),    // 10110100
	(0, 1),    // 10110101
	(0, 1),    // 10110110
	(0, 1),    // 10110111
	(0, 1),    // 10111000
	(0, 1),    // 10111001
	(0, 1),    // 10111010
	(0, 1),    // 10111011
	(0, 1),    // 10111100
	(0, 1),    // 10111101
	(0, 1),    // 10111110
	(0, 1),    // 10111111
	(0, 1),    // 11000000
	(0, 1),    // 11000001
	(0, 1),    // 11000010
	(0, 1),    // 11000011
	(0, 1),    // 11000100
	(0, 1),    // 11000101
	(0, 1),    // 11000110
	(0, 1),    // 11000111
	(0, 1),    // 11001000
	(0, 1),    // 11001001
	(0, 1),    // 11001010
	(0, 1),    // 11001011
	(0, 1),    // 11001100
	(0, 1),    // 11001101
	(0, 1),    // 11001110
	(0, 1),    // 11001111
	(0, 1),    // 11010000
	(0, 1),    // 11010001
	(0, 1),    // 11010010
	(0, 1),    // 11010011
	(0, 1),    // 11010100
	(0, 1),    // 11010101
	(0, 1),    // 11010110
	(0, 1),    // 11010111
	(0, 1),    // 11011000
	(0, 1),    // 11011001
	(0, 1),    // 11011010
	(0, 1),    // 11011011
	(0, 1),    // 11011100
	(0, 1),    // 11011101
	(0, 1),    // 11011110
	(0, 1),    // 11011111
	(0, 1),    // 11100000
	(0, 1),    // 11100001
	(0, 1),    // 11100010
	(0, 1),    // 11100011
	(0, 1),    // 11100100
	(0, 1),    // 11100101
	(0, 1),    // 11100110
	(0, 1),    // 11100111
	(0, 1),    // 11101000
	(0, 1),    // 11101001
	(0, 1),    // 11101010
	(0, 1),    // 11101011
	(0, 1),    // 11101100
	(0, 1),    // 11101101
	(0, 1),    // 11101110
	(0, 1),    // 11101111
	(0, 1),    // 11110000
	(0, 1),    // 11110001
	(0, 1),    // 11110010
	(0, 1),    // 11110011
	(0, 1),    // 11110100
	(0, 1),    // 11110101
	(0, 1),    // 11110110
	(0, 1),    // 11110111
	(0, 1),    // 11111000
	(0, 1),    // 11111001
	(0, 1),    // 11111010
	(0, 1),    // 11111011
	(0, 1),    // 11111100
	(0, 1),    // 11111101
	(0, 1),    // 11111110
	(0, 1),    // 11111111
];

pub const ZETA3_M2L_TABLE: [(u8, u8); 256] = [
	(255, 0),    // 00000000
	(255, 0),    // 00000001
	(255, 0),    // 00000010
	(255, 0),    // 00000011
	(255, 0),    // 00000100
	(255, 0),    // 00000101
	(255, 0),    // 00000110
	(255, 0),    // 00000111
	(255, 0),    // 00001000
	(255, 0),    // 00001001
	(255, 0),    // 00001010
	(255, 0),    // 00001011
	(255, 0),    // 00001100
	(255, 0),    // 00001101
	(255, 0),    // 00001110
	(255, 0),    // 00001111
	(255, 0),    // 00010000
	(255, 0),    // 00010001
	(255, 0),    // 00010010
	(255, 0),    // 00010011
	(255, 0),    // 00010100
	(255, 0),    // 00010101
	(255, 0),    // 00010110
	(255, 0),    // 00010111
	(255, 0),    // 00011000
	(255, 0),    // 00011001
	(255, 0),    // 00011010
	(255, 0),    // 00011011
	(255, 0),    // 00011100
	(255, 0),    // 00011101
	(255, 0),    // 00011110
	(255, 0),    // 00011111
	(255, 0),    // 00100000
	(255, 0),    // 00100001
	(255, 0),    // 00100010
	(255, 0),    // 00100011
	(255, 0),    // 00100100
	(255, 0),    // 00100101
	(255, 0),    // 00100110
	(255, 0),    // 00100111
	(255, 0),    // 00101000
	(255, 0),    // 00101001
	(255, 0),    // 00101010
	(255, 0),    // 00101011
	(255, 0),    // 00101100
	(255, 0),    // 00101101
	(255, 0),    // 00101110
	(255, 0),    // 00101111
	(255, 0),    // 00110000
	(255, 0),    // 00110001
	(255, 0),    // 00110010
	(255, 0),    // 00110011
	(255, 0),    // 00110100
	(255, 0),    // 00110101
	(255, 0),    // 00110110
	(255, 0),    // 00110111
	(255, 0),    // 00111000
	(255, 0),    // 00111001
	(255, 0),    // 00111010
	(255, 0),    // 00111011
	(255, 0),    // 00111100
	(255, 0),    // 00111101
	(255, 0),    // 00111110
	(255, 0),    // 00111111
	(7, 7),    // 01000000
	(7, 7),    // 01000001
	(8, 7),    // 01000010
	(8, 7),    // 01000011
	(9, 7),    // 01000100
	(9, 7),    // 01000101
	(10, 7),    // 01000110
	(10, 7),    // 01000111
	(11, 7),    // 01001000
	(11, 7),    // 01001001
	(12, 7),    // 01001010
	(12, 7),    // 01001011
	(13, 7),    // 01001100
	(13, 7),    // 01001101
	(14, 7),    // 01001110
	(14, 7),    // 01001111
	(15, 8),    // 01010000
	(16, 8),    // 01010001
	(17, 8),    // 01010010
	(18, 8),    // 01010011
	(19, 8),    // 01010100
	(20, 8),    // 01010101
	(21, 8),    // 01010110
	(22, 8),    // 01010111
	(23, 8),    // 01011000
	(24, 8),    // 01011001
	(25, 8),    // 01011010
	(26, 8),    // 01011011
	(27, 8),    // 01011100
	(28, 8),    // 01011101
	(29, 8),    // 01011110
	(30, 8),    // 01011111
	(31, 8),    // 01100000
	(32, 8),    // 01100001
	(33, 8),    // 01100010
	(34, 8),    // 01100011
	(35, 8),    // 01100100
	(36, 8),    // 01100101
	(37, 8),    // 01100110
	(38, 8),    // 01100111
	(39, 8),    // 01101000
	(40, 8),    // 01101001
	(41, 8),    // 01101010
	(42, 8),    // 01101011
	(43, 8),    // 01101100
	(44, 8),    // 01101101
	(45, 8),    // 01101110
	(46, 8),    // 01101111
	(47, 8),    // 01110000
	(48, 8),    // 01110001
	(49, 8),    // 01110010
	(50, 8),    // 01110011
	(51, 8),    // 01110100
	(52, 8),    // 01110101
	(53, 8),    // 01110110
	(54, 8),    // 01110111
	(55, 8),    // 01111000
	(56, 8),    // 01111001
	(57, 8),    // 01111010
	(58, 8),    // 01111011
	(59, 8),    // 01111100
	(60, 8),    // 01111101
	(61, 8),    // 01111110
	(62, 8),    // 01111111
	(0, 3),    // 10000000
	(0, 3),    // 10000001
	(0, 3),    // 10000010
	(0, 3),    // 10000011
	(0, 3),    // 10000100
	(0, 3),    // 10000101
	(0, 3),    // 10000110
	(0, 3),    // 10000111
	(0, 3),    // 10001000
	(0, 3),    // 10001001
	(0, 3),    // 10001010
	(0, 3),    // 10001011
	(0, 3),    // 10001100
	(0, 3),    // 10001101
	(0, 3),    // 10001110
	(0, 3),    // 10001111
	(0, 3),    // 10010000
	(0, 3),    // 10010001
	(0, 3),    // 10010010
	(0, 3),    // 10010011
	(0, 3),    // 10010100
	(0, 3),    // 10010101
	(0, 3),    // 10010110
	(0, 3),    // 10010111
	(0, 3),    // 10011000
	(0, 3),    // 10011001
	(0, 3),    // 10011010
	(0, 3),    // 10011011
	(0, 3),    // 10011100
	(0, 3),    // 10011101
	(0, 3),    // 10011110
	(0, 3),    // 10011111
	(1, 4),    // 10100000
	(1, 4),    // 10100001
	(1, 4),    // 10100010
	(1, 4),    // 10100011
	(1, 4),    // 10100100
	(1, 4),    // 10100101
	(1, 4),    // 10100110
	(1, 4),    // 10100111
	(1, 4),    // 10101000
	(1, 4),    // 10101001
	(1, 4),    // 10101010
	(1, 4),    // 10101011
	(1, 4),    // 10101100
	(1, 4),    // 10101101
	(1, 4),    // 10101110
	(1, 4),    // 10101111
	(2, 4),    // 10110000
	(2, 4),    // 10110001
	(2, 4),    // 10110010
	(2, 4),    // 10110011
	(2, 4),    // 10110100
	(2, 4),    // 10110101
	(2, 4),    // 10110110
	(2, 4),    // 10110111
	(2, 4),    // 10111000
	(2, 4),    // 10111001
	(2, 4),    // 10111010
	(2, 4),    // 10111011
	(2, 4),    // 10111100
	(2, 4),    // 10111101
	(2, 4),    // 10111110
	(2, 4),    // 10111111
	(3, 4),    // 11000000
	(3, 4),    // 11000001
	(3, 4),    // 11000010
	(3, 4),    // 11000011
	(3, 4),    // 11000100
	(3, 4),    // 11000101
	(3, 4),    // 11000110
	(3, 4),    // 11000111
	(3, 4),    // 11001000
	(3, 4),    // 11001001
	(3, 4),    // 11001010
	(3, 4),    // 11001011
	(3, 4),    // 11001100
	(3, 4),    // 11001101
	(3, 4),    // 11001110
	(3, 4),    // 11001111
	(4, 4),    // 11010000
	(4, 4),    // 11010001
	(4, 4),    // 11010010
	(4, 4),    // 11010011
	(4, 4),    // 11010100
	(4, 4),    // 11010101
	(4, 4),    // 11010110
	(4, 4),    // 11010111
	(4, 4),    // 11011000
	(4, 4),    // 11011001
	(4, 4),    // 11011010
	(4, 4),    // 11011011
	(4, 4),    // 11011100
	(4, 4),    // 11011101
	(4, 4),    // 11011110
	(4, 4),    // 11011111
	(5, 4),    // 11100000
	(5, 4),    // 11100001
	(5, 4),    // 11100010
	(5, 4),    // 11100011
	(5, 4),    // 11100100
	(5, 4),    // 11100101
	(5, 4),    // 11100110
	(5, 4),    // 11100111
	(5, 4),    // 11101000
	(5, 4),    // 11101001
	(5, 4),    // 11101010
	(5, 4),    // 11101011
	(5, 4),    // 11101100
	(5, 4),    // 11101101
	(5, 4),    // 11101110
	(5, 4),    // 11101111
	(6, 4),    // 11110000
	(6, 4),    // 11110001
	(6, 4),    // 11110010
	(6, 4),    // 11110011
	(6, 4),    // 11110100
	(6, 4),    // 11110101
	(6, 4),    // 11110110
	(6, 4),    // 11110111
	(6, 4),    // 11111000
	(6, 4),    // 11111001
	(6, 4),    // 11111010
	(6, 4),    // 11111011
	(6, 4),    // 11111100
	(6, 4),    // 11111101
	(6, 4),    // 11111110
	(6, 4),    // 11111111
];

