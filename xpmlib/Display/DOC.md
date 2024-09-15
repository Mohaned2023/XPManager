# Dispaly:
A set of clesses that use Dispaly message to the console.


# Class: `DisplayPassword`

The `DisplayPassword` class extends the `Display` class and is designed to visually present passwords stored in a dictionary format. It offers features such as displaying passwords in a table format or a natural list and logging individual passwords.

### Import Dependencies

Before using the `DisplayPassword` class, ensure the following dependencies are available:
- `Check` from the `TypeChecker` module for type validation.
- `Console` and `Table` from the `rich` module for table rendering.
- `EmptyDataError` from the `Error` module to handle empty database errors.
- `Color` from the `Color` module for color-coding output.
- The `Display` base class for inheritance.

## Constructor: `__init__(self, passwords: dict) -> None`

### Args:
- `passwords` (dict): A dictionary where keys represent account names and values represent the passwords.

### Raises:
- `EmptyDataError`: If the `passwords` dictionary is empty.

The constructor validates the type of the `passwords` argument using the `Check` module. It also raises an error if the dictionary is empty.

```python
passwords = {"email": "example123", "bank": "securepassword!"}
display = DisplayPassword(passwords)
```

## Method: `table(self) -> None`

This method displays all stored passwords in a formatted table using the `rich` module.

### Process:
1. Creates a `Table` object with columns "N" (Number), "Name" (Account Name), and "Password".
2. Loops through the `passwords` dictionary and adds rows to the table, truncating passwords longer than 30 characters.
3. Prints the formatted table to the console.

```python
display.table()
```

### Example Output:

```
+---+-------+---------------------------+
| N | Name  | Password                  |
+---+-------+---------------------------+
| 1 | email | example123                |
| 2 | bank  | securepassword!           |
+---+-------+---------------------------+
```

## Method: `natural(self) -> None`

This method displays the stored passwords in a natural list format.

### Process:
1. Loops through the `passwords` dictionary and prints each key-value pair in a numbered list with colored output.

```python
display.natural()
```

### Example Output:

```
1- email: example123
2- bank: securepassword!
```

### Static Method: `log(password: str) -> None`

This static method logs a single password to the console with color formatting.

### Args:
- `password` (str): The password to be logged.

```python
DisplayPassword.log("example123")
```

### Example Output:

```
Password: example123
```

### Usage Example

```python
from xpmlib.Display import DisplayPassword

# Initialize with a dictionary of passwords
passwords = {"email": "example123", "bank": "securepassword!"}
display = DisplayPassword(passwords)

# Display passwords in a table format
display.table()

# Display passwords in a natural list format
display.natural()

# Log a single password
DisplayPassword.log("example123")
```

## Notes
- **Password Length**: In table format, passwords longer than 30 characters are truncated for display purposes.
- **Color Output**: The class uses custom colors for visual clarity, defined by the `Color` module.