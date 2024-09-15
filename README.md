# XPManager:

A tool that uses three features.

1. Password Management tool.
2. File and Folder Encryption Management tool.
3. Backup and Recovery Management tool.

# Indexes:

- [Main](./README.md):
    - [Main](./xpm.py).

- [PasswordManagement](./xpmlib/PasswordManagement/DOC.md):
    - [GeneratePassword](./xpmlib/PasswordManagement/generate.py).
    - [GetPassword](./xpmlib/PasswordManagement/get.py).
    - [ModifyPassword](./xpmlib/PasswordManagement/modify.py).
    - [SavePassword](./xpmlib/PasswordManagement/save.py).

- [FileEncryption](./xpmlib/FileEncryption/DOC.md):
    - [CheckFile](./xpmlib/FileEncryption/check.py).
    - [Decryption](./xpmlib/FileEncryption/decryption.py).
    - [Encrytion](./xpmlib/FileEncryption/encryption.py).

- [Upload](./xpmlib/Upload/DOC.md):
    - [MegaUploader](./xpmlib/Upload/mega.py).

- [Tar](./xpmlib/Tar/DOC.md):
    - [Tar](./xpmlib/Tar/tar.py).
    - [TarExtract](./xpmlib/Tar/extract.py).

- [File](./xpmlib/File/DOC.md):
    - [FileReader](./xpmlib/File/Read.py).
    - [WriteFile](./xpmlib/File/Write.py).
    - [GetPath](./xpmlib/File/Paths.py).

- [Error](./xpmlib/Error/DOC.md):
    - [Error](./xpmlib/Error/__init__.py).
    - [NotFoundError](./xpmlib/Error/__init__.py).
    - [LengthError](./xpmlib/Error/__init__.py).
    - [ParameterError](./xpmlib/Error/__init__.py).
    - [FoundError](./xpmlib/Error/__init__.py).
    - [ArgumentNotFoundError](./xpmlib/Error/__init__.py).
    - [EmptyDataError](./xpmlib/Error/__init__.py).
    - [FileSizeError](./xpmlib/Error/__init__.py).

- [Display](./xpmlib/Display/DOC.md):
    - [Display](./xpmlib/Display/display.py).
    - [DisplayEnOrDe](./xpmlib/Display/enorde.py).
    - [DisplayError](./xpmlib/Display/error.py).
    - [DisplayKey](./xpmlib/Display/key.py).
    - [DisplayPassword](./xpmlib/Display/password.py).
    - [DisplayRemoving](./xpmlib/Display/removing.py).
    - [DisplaySaved](./xpmlib/Display/saved.py).
    - [DisplaySetting](./xpmlib/Display/setting.py).

- [ctype](./xpmlib/ctype/DOC.md):
    - [Check](./xpmlib/ctype/TypeChecker.py).

- [Color](./xpmlib/Color/DOC.md):
    - [Color](./xpmlib/Color/__init__.py).

# Information:
Number of classes: `32` class..

Time taken to complete: `10` hours..

> Made with full love by `Mohaned Sherhan Mr.x`

# Set Up:
```Bash
git clone https://github.com/Mohaned2023/XPManager.git
cd XPManager
pip install -r requirements.txt
pip install --upgrade tenacity 
python xpm.py
```
# Refernces:
- [cryptography - Fernet _cryptography.io_](https://cryptography.io/en/latest/fernet/)
- [string _docs.python.org_](https://docs.python.org/3/library/string.html)
- [random - choice _docs.python.org_](https://docs.python.org/3/library/random.html#random.choice)
- [mega _pypi.org_](https://pypi.org/project/mega.py/)
- [tarfile _docs.python.org_](https://docs.python.org/3/library/tarfile.html)
- [rich - table - console _rich.readthedocs.io_](https://rich.readthedocs.io/en/stable/introduction.html)