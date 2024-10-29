# FileEncryption:
A set of clesses that use to Check, Encrypt and Decrypt files and folders.

# Indexes:
- [Class: `CheckFile`](#class-checkfile)
- [Class: `Decryption`](#class-decryption)
- [Class: `Encrytion`](#class-encrytion)
--- 
# Class: `CheckFile`

This class provides methods to check if a file or a database within a folder is encrypted.

### Import Dependencies

Before using the `CheckFile` class, ensure you have the following dependencies in your project:
- `Check` from the `TypeChecker` module (used for type validation).
- `exists`, `isdir`, and `join` from Python’s `os.path` module.
- `GetPath` from the `File.Paths` module (used to fetch all file paths within a directory).

## Constructor: `__init__(self, path: str) -> None`

### Args:
- `path` (str): The file or directory path that will be used for checks.

The constructor validates the `path` type to ensure it is a string and then stores it as an internal property.

```python
file_checker = CheckFile(path="/path/to/directory_or_file")
```

## Method: `isencryption(self) -> bool`

This method checks if the file is encrypted based on its name.

### Returns:
- `bool`: 
  - `True` if the file appears to be encrypted (if the file extension ends with 'x').
  - `False` if not encrypted.

### Raises:
- `FileNotFoundError`: If the file does not exist.

```python
is_encrypted = file_checker.isencryption()
print(is_encrypted)
```

## Method: `isdatabaseencryption(self, file_name: str) -> bool`

This method checks if a specific file in the database (directory) is encrypted.

### Args:
- `file_name` (str): The name of the file (without the extension) to check within the directory.

### Returns:
- `bool`: 
  - `True` if the file exists in the directory and is encrypted (i.e., the file has an extension ending with 'x').
  - `False` if not encrypted.

### Raises:
- `NotADirectoryError`: If the provided path is not a directory.

```python
is_db_encrypted = file_checker.isdatabaseencryption(file_name="database_file")
print(is_db_encrypted)
```

### Usage Example

```python
from xpmlib.FileEncryption import CheckFile

# Initialize the CheckFile object with a directory path
file_checker = CheckFile(path="/path/to/directory")

# Check if a file is encrypted
try:
    if file_checker.isencryption():
        print("File is encrypted.")
    else:
        print("File is not encrypted.")
except FileNotFoundError as e:
    print(e)

# Check if a specific file in the directory is encrypted
try:
    if file_checker.isdatabaseencryption("database_file"):
        print("Database file is encrypted.")
    else:
        print("Database file is not encrypted.")
except NotADirectoryError as e:
    print(e)
```

## NOTE
- This class assumes the encryption status of a file is determined by the file extension ending with 'x'.
-  Make sure the path passed to the `CheckFile` class is valid, and the file or directory exists to avoid raising errors.

---
# Class: `Decryption`

The `Decryption` class is responsible for decrypting files based on a given key and managing file removal after decryption if required.

### Import Dependencies

Before using the `Decryption` class, ensure you have the following dependencies in your project:
- `Fernet` from the `cryptography.fernet` module (used for encryption and decryption).
- `remove` from Python’s `os` module (used to delete files).
- Custom modules:
  - `DisplayRemoving`, `DisplaySetting`, and `DisplayEnOrDe` for logging actions.
  - `Check` from the `TypeChecker` module for type validation.
  - `FileReader` for reading files.
  - `WriteFile` for writing files.
  - `Color` for color-coded console outputs.

## Constructor: `__init__(self, paths: list[str], key: bytes, remove_files: bool = False) -> None`

### Args:
- `paths` (list[str]): A list of file paths (strings) to decrypt.
- `key` (bytes): The decryption key (must be a valid key used to encrypt the files).
- `remove_files` (bool, optional): A flag to indicate whether to remove the encrypted files after decryption. Defaults to `False`.

The constructor validates the type of `paths` and initializes the provided values for further use in decryption.

```python
decryptor = Decryption(paths=["/path/to/file1.x", "/path/to/file2.x"], key=b'your_decryption_key', remove_files=True)
```

## Method: `decrypt(self) -> str`

This method decrypts the files in the provided `paths`.

### Returns:
- `str`: The path of the last decrypted file.

### Process:
1. Iterates over the list of paths and checks if the file is encrypted (based on the file name ending with 'x').
2. If the file is not encrypted, it skips the file and displays a message.
3. For encrypted files:
   - Reads the encrypted data using `FileReader`.
   - Decrypts the data using the `Fernet` decryption algorithm.
   - Writes the decrypted data to a new file using `WriteFile`.
   - If `remove_files` is set to `True`, it deletes the original encrypted file.
4. Logs all decryption and file-removal actions.

```python
last_decrypted_path = decryptor.decrypt()
print(f"Last decrypted file: {last_decrypted_path}")
```

### Usage Example

```python
from xpmlib.FileEncryption import Decryption

# Initialize the Decryption object with the file paths, decryption key, and option to remove the files after decryption
decryptor = Decryption(paths=["/path/to/file1.x", "/path/to/file2.x"], key=b'your_decryption_key', remove_files=True)

# Decrypt the files and get the path of the last decrypted file
try:
    last_decrypted_file = decryptor.decrypt()
    print(f"Decryption successful! Last decrypted file: {last_decrypted_file}")
except Exception as e:
    print(f"Error during decryption: {e}")
```

## Notes
- **File Encryption Check**: Files are considered encrypted if their names end with an 'x'. If the file does not meet this condition, it is skipped.
- **Decryption Key**: Ensure that the key passed to the `Decryption` class is the correct key used for encrypting the files.
- **Removing Encrypted Files**: If `remove_files` is set to `True`, the original encrypted files will be deleted after decryption. Ensure you have backups if necessary.

--- 
# Class: `Encrytion`

The `Encrytion` class is designed to encrypt files using the `Fernet` encryption algorithm and offers the ability to remove the original files after encryption.

### Import Dependencies

Before using the `Encrytion` class, ensure the following modules are available in your project:
- `Fernet` from the `cryptography.fernet` module (for encryption).
- `remove` from Python's `os` module (for file removal).
- Custom modules:
  - `DisplaySetting`, `DisplayRemoving`, `DisplayKey`, `DisplayEnOrDe` for logging actions.
  - `FileReader` for reading files.
  - `WriteFile` for writing files.
  - `Check` from the `TypeChecker` module for type validation.
  - `Color` for console color output.

## Constructor: `__init__(self, paths: list[str], key: bytes = None, remove_files: bool = False) -> None`

### Args:
- `paths` (list[str]): A list of file paths (strings) to encrypt.
- `key` (bytes, optional): The encryption key to use. If not provided, a new key is generated using `Fernet.generate_key()`. Defaults to `None`.
- `remove_files` (bool, optional): Flag to indicate whether the original files should be removed after encryption. Defaults to `False`.

The constructor validates the `paths` parameter to ensure it is a list, initializes the provided encryption key, or generates one if none is provided. It also stores the option for removing files.

```python
encryptor = Encrytion(paths=["/path/to/file1", "/path/to/file2"], key=b'your_encryption_key', remove_files=True)
```

## Method: `encrypt(self) -> bytes`

This method encrypts the files listed in the `paths` parameter.

### Returns:
- `bytes`: The encryption key.

### Process:
1. For each file in `paths`, it appends an '.x' to the filename to indicate that the file is encrypted.
2. Reads the file content using `FileReader`.
3. Logs the encryption process via `DisplayEnOrDe`.
4. Encrypts the file data using the `Fernet` algorithm and the provided key.
5. Writes the encrypted data to a new file with the `.x` extension using `WriteFile`.
6. Logs the new file creation via `DisplaySetting`.
7. If `remove_files` is set to `True`, the original file is deleted using `remove()`, and the action is logged via `DisplayRemoving`.
8. The encryption key is displayed using `DisplayKey`.

```python
key = encryptor.encrypt().decode()
print(f"{key = }")
```

### Usage Example

```python
from xpmlib.FileEncryption import Encrytion

# Initialize the Encrytion object with the file paths, encryption key, and option to remove the original files after encryption
encryptor = Encrytion(paths=["/path/to/file1", "/path/to/file2"], key=b'your_encryption_key', remove_files=True)

# Encrypt the files
encryptor.encrypt()

# Output the encryption key
print(f"Encryption Key: {encryptor._Encrytion__key.decode()}")
```

## Notes
- **File Encryption**: Files are encrypted by appending `.x` to their names.
- **Encryption Key**: Ensure you securely store the key used for encryption. It is required for decrypting the files.
- **Removing Files**: If `remove_files` is set to `True`, the original unencrypted files will be deleted after encryption. Be cautious and create backups if necessary.