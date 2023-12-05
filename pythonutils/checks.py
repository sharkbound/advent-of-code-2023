__all__ = [
    'is_valid_array_index'
]


def is_valid_array_index(arr, y, x):
    return 0 <= y < arr.shape[0] and 0 <= x < arr.shape[1]
