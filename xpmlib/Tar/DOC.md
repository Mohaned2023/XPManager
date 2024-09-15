# Tar:
A set of clesses that compress and decompress folders.


# Indexes:
- [Class: `Tar`](#class-tar):
    - [Import Dependencies](#import-dependencies).
    - [Args](#args).
    - [Raises](#raises).
    - [Process](#process).
    - [Returns](#returns).
    - [Usage Example](#usage-example).
    - [Notes](#notes).
- [Class: `TarExtract`](#class-tarextract): 
    - [Import Dependencies](#import-dependencies-1).
    - [Args](#args-1).
    - [Raises](#raises-1).
    - [Process](#process-1).
    - [Usage Example](#usage-example-1).
    - [Notes](#notes-1).

---
# Class: `Tar`

The `Tar` class is designed to compress a folder into a `.tar.gz` file. It ensures that both the source folder and the destination path exist before proceeding with the compression.

### Import Dependencies

Before using the `Tar` class, ensure the following dependencies are available:
- `Check` from the `TypeChecker` module for type validation.
- `Color` for console color output (optional for logging).
- `basename`, `isdir`, `join` from Python’s `os.path` module for handling file paths and directory checking.
- `listdir` from the `os` module for listing directory contents.
- `taropen` from Python’s `tarfile` module to create a `.tar.gz` archive.

## Constructor: `__init__(self, folder_path: str, save_path: str) -> None`

### Args:
- `folder_path` (str): The path of the folder to be compressed.
- `save_path` (str): The path where the resulting `.tar.gz` file will be saved.

### Raises:
- `NotADirectoryError`: If either `folder_path` or `save_path` is not a valid directory.

The constructor validates the input types for `folder_path` and `save_path` and checks if both paths are valid directories. If the directories are invalid, an error is raised.

```python
compressor = Tar(folder_path="/path/to/folder", save_path="/path/to/save")
```

## Method: `folder(self) -> str`

This method compresses the folder specified by `folder_path` into a `.tar.gz` archive and saves it in the directory specified by `save_path`.

### Process:
1. Extracts the folder name from the `folder_path`.
2. Creates the output file path by appending `.tar.gz` to the folder name.
3. Opens a tar file in write mode (`'w:gz'`) and adds the folder contents to the archive.
4. Returns the path of the created `.tar.gz` file.

### Returns:
- `str`: The path to the `.tar.gz` file created.

```python
archive_path = compressor.folder()
print(f"Folder compressed and saved to: {archive_path}")
```

### Usage Example

```python
from xpmlib.Tar import Tar

# Initialize the Tar object with the folder path and save path
compressor = Tar(folder_path="/path/to/folder", save_path="/path/to/save")

# Compress the folder and get the output file path
archive_path = compressor.folder()

# Output the result
print(f"Folder compressed to: {archive_path}")
```

## Notes
- **Folder Compression**: The method compresses the entire contents of the folder (including subdirectories) into a `.tar.gz` file.
- **File Naming**: The output file is named based on the folder’s name, followed by `.tar.gz`.
- **Valid Directories**: Ensure that both `folder_path` and `save_path` are valid directories. If not, a `NotADirectoryError` is raised.



---
# Class: `TarExtract`

The `TarExtract` class is designed to extract files from a `.tar.gz` archive to a specified directory. It checks the existence of the `.tar.gz` file before extraction.

### Import Dependencies

Before using the `TarExtract` class, ensure the following dependencies are available:
- `Check` from the `TypeChecker` module for type validation.
- `isfile`, `join`, `basename` from Python’s `os.path` module for handling file paths and file existence checking.
- `taropen` from Python’s `tarfile` module to handle extraction of the `.tar.gz` archive.
- `Color` for optional colored logging to the console.

## Constructor: `__init__(self, file_path: str, extract_path: str) -> None`

### Args:
- `file_path` (str): The path of the `.tar.gz` file to be extracted.
- `extract_path` (str): The directory where the files will be extracted.

### Raises:
- `FileNotFoundError`: If the specified `file_path` does not exist or is not a file.

The constructor validates the input types for `file_path` and `extract_path` and checks if the `file_path` is a valid file. If the file is invalid, an error is raised.

```python
extractor = TarExtract(file_path="/path/to/file.tar.gz", extract_path="/path/to/extract")
```

## Method: `extract(self) -> None`

This method extracts the contents of the `.tar.gz` file to the specified `extract_path`.

### Process:
1. Prints the file being extracted using color-coded output.
2. Opens the `.tar.gz` file in read mode (`'r:gz'`).
3. Extracts all files in the archive to the specified extraction directory.
4. Prints the extraction success message.

```python
extractor.extract()
```

### Usage Example

```python
from xpmlib.Tar import TarExtract

# Initialize the TarExtract object with the file path and extraction path
extractor = TarExtract(file_path="/path/to/file.tar.gz", extract_path="/path/to/extract")

# Extract the .tar.gz file
extractor.extract()
```

## Notes
- **File Existence**: Ensure the `file_path` points to a valid `.tar.gz` file, otherwise a `FileNotFoundError` will be raised.
- **Extract Destination**: The files are extracted to the directory specified by `extract_path`.
- **Extract All**: The method extracts all contents of the `.tar.gz` file without any filters.