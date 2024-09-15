# Upload:
A sef of clesses that upload the backup.

# Indexes:
- [Class: `MegaUploader`](#class-megauploader):
    - [Import Dependencies](#import-dependencies).
    - [Args](#args).
    - [Raises](#raises).
    - [Process](#process).
    - [Raises](#raises-1).
    - [Usage Example](#usage-example).
    - [Notes](#notes)
---
# Class: `MegaUploader`

The `MegaUploader` class is responsible for uploading a file to the `mega.nz` cloud storage. It also validates whether the user has enough free storage space in their cloud account before initiating the upload.

### Import Dependencies

Before using the `MegaUploader` class, ensure you have the following dependencies:
- `Check` from the `TypeChecker` module for type validation.
- `isfile` and `getsize` from Python’s `os.path` module for file existence and size checks.
- `Mega` from the `mega` module for interacting with the `mega.nz` API.
- `FileSizeError` from the `Error` module for raising exceptions when the file size exceeds available cloud storage.
- `Color` for color-coded console outputs.

## Constructor: `__init__(self, file_path: str, email: str, password: str) -> None`

### Args:
- `file_path` (str): The local path of the file to be uploaded.
- `email` (str): The email address associated with the `mega.nz` account.
- `password` (str): The password for the `mega.nz` account.

### Raises:
- `FileNotFoundError`: If the provided `file_path` does not exist or is not a file.

The constructor validates the types of `file_path`, `email`, and `password` and ensures that the file exists. It then logs in to the user's `mega.nz` account and retrieves the available storage space in gigabytes.

```python
uploader = MegaUploader(file_path="/path/to/file", email="your_email@example.com", password="your_password")
```

## Method: `upload_file(self) -> None`

This method uploads the file to the `mega.nz` cloud storage.

### Process:
1. Calculates the available free space in the cloud account.
2. Calculates the size of the file to be uploaded.
3. Compares the file size with the available cloud storage.
4. If there is enough storage space, it uploads the file.
5. If the file size exceeds available storage, it raises a `FileSizeError`.

### Raises:
- `FileSizeError`: If the file size exceeds the available storage space in the `mega.nz` account.

```python
uploader.upload_file()
```

### Usage Example

```python
from xpmlib.Upload  import MegaUploader

# Initialize the MegaUploader with the file path, email, and password
uploader = MegaUploader(file_path="/path/to/file", email="your_email@example.com", password="your_password")

# Upload the file
try:
    uploader.upload_file()
    print("File uploaded successfully!")
except FileNotFoundError as e:
    print(f"Error: {e}")
except FileSizeError as e:
    print(f"Error: {e}")
```

## Notes
- **File Existence**: Ensure that the file path passed to the `MegaUploader` class points to an existing file.
- **Cloud Storage Check**: The class checks whether there is enough free space in the `mega.nz` cloud account before attempting to upload the file.
- **API Login**: The class logs in to the `mega.nz` account using the provided email and password.
- **File Size Calculation**: The file size is calculated in gigabytes, and if it exceeds the available storage, the upload is not performed.