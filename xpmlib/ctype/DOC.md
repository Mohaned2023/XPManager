# ctype:
A class that use to check the type of the value.

# Class: `Check`

This class contains a static method `check_type` which verifies the types of values in a list against corresponding expected types. It's primarily used for input validation in various parts of a program.

## Method: `check_type(*, values: list, types: list[type]) -> bool`

### Args:
- `values` (list): A list of values to be checked.
- `types` (list[type]): A list of expected types corresponding to the `values`.

### Raises:
- `TypeError`: Raised when the type of any value does not match the expected type.

### Returns:
- `bool`: Returns `True` if all values match their expected types.

### Method Explanation:
- The method iterates over the `values` and `types` simultaneously using `zip()`.
- For each pair of `value` and `type`, it checks if the value is an instance of the expected type using `isinstance()`.
- If a mismatch is found, a `TypeError` is raised with a descriptive error message.

### Usage Example

```python
from xpmlib.ctype import Check

# Example 1: Valid types
values = ["example", 42]
types = [str, int]

# This should pass without raising an exception.
Check.check_type(values=values, types=types)

# Example 2: Invalid types
values = ["example", "wrong_type"]
types = [str, int]

# This will raise a TypeError because the second value is a string but expected to be an integer.
Check.check_type(values=values, types=types)
```

### Error Handling Example

If the second value in the `values` list does not match the expected type (in this case `int`), the following error will be raised:

```bash
TypeError: Expect `int` found str!
```

## Usage Notes:
- **Positional Arguments**: The method requires the use of keyword arguments (`values` and `types`) to ensure clarity when calling the function.
- **Type Matching**: If any type mismatch occurs, the error message will provide the expected type and the actual type found, which helps with debugging and validation.