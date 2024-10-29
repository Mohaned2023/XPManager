# XPManager:
Many times I have very sensitive information and I do not want to upload it\
to large companies or use their protection methods, so I had the idea of ​\
​programming this modest tool to secure my files and also to manage my \
passwords in a strong way in terms of creation, security, etc.

In addition, I found that I was learning the principle of OOP using Python.\
At the end of the course, I was asked to do a final project for the course,\
so I found that this was the perfect opportunity to realize this idea,\
knowing that the code lacks many principles. I had a tight time to deliver\
the project. I did not make the code 100% clean and apply all the concepts\
of OOP, but I learned a lot of things.

# Installation:
```Bash
$ git clone https://github.com/Mohaned2023/XPManager.git
$ cd XPManager
$ pip install -r requirements.txt
$ pip install --upgrade tenacity 
$ python xpm.py
```

# Usage:
The tool is built on three sub-tools, which are:
1. Password Management.
2. File Management.
3. Backup Management.

Here is an explanation of how to use each one:
### Password Management:
to use this tool set `-pm` or `--password-management`\
followed by: 
- `-l [number]` or `--length [number]` for creating a password with length.\
example:
```bash
$ python xpm.py -pm -l 32
```
- `-s` or `--save` for saving the password in the json file.\
You will be asked if you want to encryot the json file.\
If you entered `yse` or `y` the tool will encrypt the file using\
the `Fernet` algorithm in `cryptography`.\
example:
```bash
$ python xpm.py -pm -l 32 -s
```
- `-up` or `--update-password` for update the password in the database.
- `-d` or `--delete-password` for deleting a password.
- `-fp` or `--find-password` for finding a password from the database.
- `-sh` or `--show` followed by: 
    - `-t` or `--table` for dispaly the passwors as table.
    - `-n` or `--natural` for dispaly the passwors as list.

### File Management:
to use this tool set `-fm` or `--file-management`\
followed by: 
- `-ef` or `--encrypt-file` for encrypt a file.
- `-df` or `--decrypt-file` for decrypt a file.
- `-efs` or `--encrypt-folder` for encrypt files in folder.
- `-dfs` or `--decrypt-folder` for decrypt files in folder.

### Backup Management:
to use this tool set `-bm` or `--backup-management`\
followed by: 
- `-pdb` or `--password-database` to backup the password database, followed by:
    - `-lb` or `--local-backup` for local backup.
    - `-mb` or `--mega-backup` upload the backup to [mega.nz](https://mega.nz).
- `-fb` or `--folder-backup` to backup a folder, followed by:
    - `-lb` or `--local-backup` for local backup.
    - `-mb` or `--mega-backup` upload the backup to [mega.nz](https://mega.nz).

example:
```bash
# Folder with local backup:
$ python xpm.py -bm -fb -lb
# Folder with mega.nz backup:
$ python xpm.py -bm -fb -mb
# Password database with loacl backup:
$ python xpm.py -bm -pdb -lb
# Password database with mega.nz backup:
$ python xpm.py -bm -pdb -mb
```

# Classes Docs:
- [PasswordManagement](./xpmlib/PasswordManagement/DOC.md)
- [FileEncryption](./xpmlib/FileEncryption/DOC.md)
- [Upload](./xpmlib/Upload/DOC.md)
- [Tar](./xpmlib/Tar/DOC.md)
- [File](./xpmlib/File/DOC.md)
- [Error](./xpmlib/Error/DOC.md)
- [Display](./xpmlib/Display/DOC.md)
- [ctype](./xpmlib/ctype/DOC.md)
- [Color](./xpmlib/Color/DOC.md)
- [Key](./xpmlib/Key/DOC.md)


# Refernces:
- [cryptography - Fernet _cryptography.io_](https://cryptography.io/en/latest/fernet/)
- [string _docs.python.org_](https://docs.python.org/3/library/string.html)
- [random - choice _docs.python.org_](https://docs.python.org/3/library/random.html#random.choice)
- [mega _pypi.org_](https://pypi.org/project/mega.py/)
- [tarfile _docs.python.org_](https://docs.python.org/3/library/tarfile.html)
- [rich - table - console _rich.readthedocs.io_](https://rich.readthedocs.io/en/stable/introduction.html)
--- 
> Made with full love by `Mohaned Sherhan Mr.x`