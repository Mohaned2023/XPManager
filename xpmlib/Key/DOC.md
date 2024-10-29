### `Key` Class

#### Purpose
The `Key` class provides secure key encoding and decoding functionality. It includes a static method to convert keys into encoded or decoded formats based on a specified secure constant.

#### Methods

##### `making`
This static method encodes or decodes a key based on an integer `n` and a boolean `secure` flag.

```python
@staticmethod
def making(*, key: bytes, n: int, secure: bool) -> bytes:
```

- **Parameters**:
    - `key` (`bytes`): The key to be encoded or decoded.
    - `n` (`int`): A secure integer constant (must be between 1000 and 9999).
    - `secure` (`bool`): A flag indicating whether to encode (`True`) or decode (`False`) the key.

- **Raises**:
    - `ValueError`: If `n` is not within the range (1000 <= n <= 9999).

- **Returns**:
    - `bytes`: The resulting encoded or decoded key as a byte sequence.

- **Functionality**:
    - When `secure` is `True`, the method encodes the key by converting each character to its ASCII value, multiplying by `n`, and converting it to a hexadecimal string.
    - When `secure` is `False`, the method decodes by reversing the encoding process, dividing the hexadecimal value by `n`, and converting it back to characters.

