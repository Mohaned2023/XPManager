## Exit Codes:
Exit codes are numerical values returned by a program upon completion, indicating the success or failure of the executed command. 
These codes are essential for automation, scripting, and error handling, allowing other programs or users to easily interpret the 
outcome of a command. By convention, an exit code of `0` signifies success, while any non-zero value indicates an error or failure. 
Properly defining and using exit codes ensures better usability and troubleshooting in command-line environments.

## XPManager Exit Codes:
In XManager we provide exit codes to facilitate error tracking and make integration with other tools easy. Here is a table showing the exit codes, their values ​​and descriptions:

| name                                              | Exit Code | Description                                                         |
|---------------------------------------------------|-----------|---------------------------------------------------------------------|
| [FileNotFound](#filenotfound)                     | 50        | The file you will be working on does not exist.                     |
| [FileCreate](#filecreate)                         | 51        | Can not create the file.                                            |
| [FileOpen](#fileopen)                             | 52        | Can not open the file.                                              |
| [FileRead](#fileread)                             | 53        | Can not read the file.                                              |
| [FileWrite](#filewrite)                           | 54        | Can not write to the file.                                          |
| [FileSeek](#fileseek)                             | 55        | Can not move the file pointer.                                      |
| [FileFlush](#fileflush)                           | 56        | Can not flush the file to the disk.                                 |
| [FileDelete](#filedelete)                         | 57        | Can not delete the file.                                            |
| [FileAlreadyEncrypted](#filealreadyencrypted)     | 58        | The file you will be working on is already encrypted.               |
| [FileNotEncrypted](#filenotencrypted)             | 59        | The file you will be working on is not encrypted.                   |
| [DirNotFound](#dirnotfound)                       | 65        | The directory you will be working on does not exist.                |
| [DirCreate](#dircreate)                           | 66        | Can not create the directory.                                       |
| [DirUnsupported](#dirunsupported)                 | 67        | Can not process the directory because it is not supported.          |
| [CanNotGetDirData](#cannotgetdirdata)             | 68        | Can not get the directory data.                                     |
| [SystemDataDirNotFound](#systemdatadirnotfound)   | 69        | Can not find the system data directory.                             |
| [CanNotGetFileOrDirType](#cannotgetfileordirtype) | 70        | The type could not be determined whether it is file or directory.   |
| [CanNotGetJsonObject](#cannotgetjsonobject)       | 75        | Can not map the json object to Value type in serde_json.            |
| [InvalidJson](#invalidjson)                       | 76        | The json value is invalid.                                          |
| [InvalidKey](#invalidkey)                         | 80        | Invalid encryption or decryption key.                               |
| [InvalidEncryptionData](#invalidencryptiondata)   | 81        | Encryption data can not be decrypted.                               |
| [DBConnection](#dbconnection)                     | 85        | Can not connect the SQLite database file.                           |
| [DBInsert](#dbinsert)                             | 86        | Can not insert to the database file.                                |
| [DBCreateTable](#dbcreatetable)                   | 87        | Can not create the database table.                                  |
| [PMDatabaseNotFound](#pmdatabasenotfound)         | 89        | Password manager database file not found.                           |
| [PMDatabaseEmpty](#pmdatabaseempty)               | 90        | Password manager database is empty.                                 |
| [LMDatabaseNotFound](#lmdatabasenotfound)         | 91        | Log manager database file not found.                                |
| [LogNotFound](#lognotfound)                       | 92        | Not found log in the database.                                      |
| [Input](#input)                                   | 95        | Error input.                                                        |
| [MissingArg](#missingarg)                         | 96        | Missing some args.                                                  |
| [ConfirmationNotMatch](#confirmationnotmatch)     | 97        | The confirmation code not match.                                    |


## Where:
#### FileNotFound
- backup manager - restore.
- encryption manager - decrypt file.
- encryption manager - encrypt file.
- filelib - wipe file.
- filelib - copy.
#### FileCreate
- filelib - create file.
#### FileOpen
- encryption manager - decrypt file.
- encryption manager - encrypt file.
#### FileRead
- encryption manager - decrypt file.
#### FileWrite
- filelib - wipe file.
#### FileSeek
- filelib - wipe file.
#### FileFlush
- filelib - wipe file.
#### FileDelete
- filelib - delete file.
#### FileAlreadyEncrypted
- password manager - encrypt.
#### FileNotEncrypted
- backup manager - backup.
- encryption manager - encrypt file.
- password manager - decrypt.
#### DirNotFound
- filelib - log.
- filelib - dir files tree.
- filelib - copy.
- filelib - password manager.
#### DirCreate
- filelib - create file.
#### DirUnsupported
- filelib - dir files tree.
#### CanNotGetDirData
- filelib - dir files tree.
#### SystemDataDirNotFound
- filelib - log.
- filelib - password manager.
#### CanNotGetFileOrDirType
- filelib - dir files tree.
#### CanNotGetJsonObject
- filelib - read json.
#### InvalidJson
- filelib - read json.
#### InvalidKey
- encryption manager - encrypt file.
- encryption manager - decrypt file.
#### InvalidEncryptionData
- encryption manager - decrypt file.
#### DBConnection
- dblib - log.
- dblib - password manager.
#### DBInsert
- dblib - log.
- dblib - password manager.
#### DBCreateTable
- dblib - log.
- dblib - password manager.
#### PMDatabaseNotFound
- backup manager - backup.
- password manager - count.
- password manager - decrypt.
- password manager - delete.
- password manager - encrypt.
- password manager - find.
- password manager - show.
- password manager - update.
#### PMDatabaseEmpty
- password manager - show.
#### LMDatabaseNotFound
- backup manager - backup.
- log manager - clear.
- log manager - delete.
- log manager - find.
- log manager - show.
#### LogNotFound
- log manager - find.
#### Input
- backup manager - restore.
- encryption manager - decrypt file.
- encryption manager - encrypt file.
- password manager - generate.
- password manager - save.
- password manager - update.
#### MissingArg
- backup manager - backup.
- backup manager - restore.
- matcheslibs - mod.
- password manager - update.
#### ConfirmationNotMatch
- utilities - confirm.