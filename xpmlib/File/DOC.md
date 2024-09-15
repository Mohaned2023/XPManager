# File:
A set of clesses that use to Read, Write and get paths of files and folders.

# Indexes:
- [Class: `FileReader`](#class-filereader):
    - [Import Dependencies](#import-dependencies).
    - [Args](#args).
    - [Process](#process).
    - [Raises](#raises).
    - [Returns](#returns).
    - [Usage Example](#usage-example).
    - [Notes](#notes).
- [Class: `WriteFile`](#class-filereader):
    - [Import Dependencies](#import-dependencies-1).
    - [Args](#args-1).
    - [Process](#process-1).
    - [Usage Example](#usage-example-1).
    - [Notes](#notes-1).
- [Class: `GetPath`](#class-getpath):
    - [Import Dependencies](#import-dependencies-2).
    - [Args](#args-2).
    - [Process](#process-2).
    - [Raises](#raises-1).
    - [Returns](#returns-1).
    - [Usage Example](#usage-example-2).
    - [Notes](#notes-2).

---
# Class: `FileReader`

The `FileReader` class is designed to read the contents of a file in binary mode. It ensures that the file exists before attempting to read it.

### Import Dependencies

Before using the `FileReader` class, ensure the following dependencies are available:
- `Check` from the `TypeChecker` module for type validation.
- `isfile` from Python’s `os.path` module for checking file existence.

## Constructor: `__init__(self, path: str) -> None`

### Args:
- `path` (str): The path of the file to be read.

The constructor validates the input type of `path` using the `Check` module. It stores the file path for later use in the `read` method.

```python
file_reader = FileReader(path="/path/to/file.txt")
```

## Method: `read(self) -> bytes`

This method reads the file specified by the `path` in binary mode and returns its contents.

### Process:
1. Checks if the file exists using `isfile`.
2. If the file does not exist, raises a `FileNotFoundError`.
3. If the file exists, it opens the file in binary mode (`'rb'`) and reads its contents.
4. Returns the file data as bytes.

### Raises:
- `FileNotFoundError`: If the file does not exist at the specified path.

### Returns:
- `bytes`: The contents of the file as a byte object.

```python
data = file_reader.read()
print(data)
```

### Usage Example

```python
from xpmlib.File import FileReader

# Initialize the FileReader with the file path
file_reader = FileReader(path="/path/to/file.txt")

# Read the file and get its contents as bytes
try:
    data = file_reader.read()
    print("File data:", data)
except FileNotFoundError as e:
    print(f"Error: {e}")
```

### Notes
- **File Existence**: The method checks if the file exists before attempting to read it. If the file does not exist, a `FileNotFoundError` is raised.
- **Binary Reading**: The file is read in binary mode, meaning the data is returned as a `bytes` object. This is useful for reading any type of file, including text, images, and binary data.


--- 
# Class: `WriteFile`

The `WriteFile` class is designed to write binary data to a file. It ensures that the file path and data provided are valid before writing.

### Import Dependencies

Before using the `WriteFile` class, ensure the following dependencies are available:
- `Check` from the `TypeChecker` module for type validation.

## Constructor: `__init__(self, path: str, data: bytes) -> None`

### Args:
- `path` (str): The path where the file will be written.
- `data` (bytes): The binary data to be written to the file.

The constructor validates the input types for `path` (as a string) and `data` (as bytes) using the `Check` module. It stores the `path` and `data` for use in the `write` method.

```python
file_writer = WriteFile(path="/path/to/file.txt", data=b"Hello, World!")
```

## Method: `write(self) -> None`

This method writes the binary data stored in the `data` attribute to the file specified by the `path`.

### Process:
1. Opens the file at the given `path` in binary write mode (`'wb'`).
2. Writes the binary `data` to the file.
3. Closes the file automatically when done.

```python
file_writer.write()
```

### Usage Example

```python
from xpmlib.File import WriteFile

# Initialize the WriteFile object with the file path and data
file_writer = WriteFile(path="/path/to/file.txt", data=b"Hello, World!")

# Write the data to the file
file_writer.write()
print("File written successfully!")
```

## Notes
- **Binary Mode**: The file is opened in binary write mode (`'wb'`), meaning the data should be in bytes format. This allows for writing any kind of file (text, image, etc.).
- **Overwriting**: If the file already exists, it will be overwritten. Ensure that the provided `path` is correct to avoid unintended data loss.


---
# Class: `GetPath`

The `GetPath` class is designed to find and return the paths of all files within a specified folder, including files in subdirectories.

### Import Dependencies

Before using the `GetPath` class, ensure the following dependencies are available:
- `Check` from the `TypeChecker` module for type validation.
- `isdir` from Python’s `os.path` module to check whether the folder exists.
- `Path` from Python’s `pathlib` module to handle directory traversal.

### Constructor: `__init__(self, folder_path: str)`

### Args:
- `folder_path` (str): The path of the folder from which to retrieve file paths.

### Raises:
- `NotADirectoryError`: If the specified `folder_path` is not a valid directory.

The constructor validates the input type for `folder_path` and checks if the provided path is a valid directory. If the path is not a directory, it raises an error.

```python
path_finder = GetPath(folder_path="/path/to/folder")
```

## Method: `get_all_files_paths(self) -> list[str]`

This method retrieves the paths of all files within the specified folder and its subdirectories.

### Process:
1. Uses the `Path` class from the `pathlib` module to traverse the directory.
2. Collects the paths of all files (ignores directories) and stores them in the `__all_paths` attribute.
3. Returns a list of file paths as strings.

### Returns:
- `list[str]`: A list of all file paths found within the directory.

```python
all_files = path_finder.get_all_files_paths()
print(all_files)
```

### Usage Example

```python
from xpmlib.File import GetPath

# Initialize the GetPath object with the folder path
path_finder = GetPath(folder_path="/path/to/folder")

# Get all file paths in the folder
all_files = path_finder.get_all_files_paths()

# Print the paths
for file_path in all_files:
    print(file_path)
```

## Notes
- **Recursive Search**: The method searches all subdirectories within the specified folder to find files.
- **File Only**: The method only returns paths of files, not directories.
- **Directory Validation**: The class checks whether the provided folder path is valid before performing the file search. If the folder does not exist, a `NotADirectoryError` is raised.