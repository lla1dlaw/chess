import numpy as np


def main():
    integer_bitboard = top_pons()
    print(f"Integer Representation of bitboard: {integer_bitboard}")
    print(f"Binary Representation of bitboard: {bin(integer_bitboard)}")


def bitboard_to_long(matrix: list[list[int]]) -> np.uint64:
    mat = np.array(matrix).flatten()
    mat = mat.astype(str)
    mat = mat.tolist()
    string_bitmap = "".join(mat)
    print(f"Flattened String Bitmap: {string_bitmap}")
    print(f"Flatened String Bitmap Length {len(string_bitmap)}\n")
    # Convert binary string to integer using base 2, then to uint64
    binary_int = int(string_bitmap, 2)
    return np.uint64(binary_int)



def top_pons() -> np.uint64:
    bitboard = [[int(i == 1)] * 8 for i in range(8)]
    return bitboard_to_long(bitboard)


if __name__ == "__main__":
    main()


