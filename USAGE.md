## Index:
- [password-manager](#password-manager)
    - [generate](#generate)
    - [save](#save)
    - [find](#find)
    - [show](#show)
    - [count](#count)
    - [update](#update)
    - [delete](#delete)
    - [encrypt](#encrypt)
    - [decrypt](#decrypt)
- [encryption-manager](#encryption-manager)
    - [encrypt-file](#encrypt-file)
    - [decrypt-file](#decrypt-file)
    - [encrypt-dir](#encrypt-dir)
    - [decrypt-dir](#decrypt-dir)
    - [encode](#encode)
    - [decode](#decode)
- [backup-manager](#backup-manager)
    - [backup](#backup)
    - [restore](#restore)
- [log-manager](#log-manager)
    - [clear](#clear)
    - [show](#show-1)
    - [find](#find-1)
    - [delete](#delete-1)

## password-manager:
Generate, save, update, delete, and manage passwords.

### generate:
Generate new password as ASCII/HEX.
```sh
# NOTE: password does NOT contain space.

# generate password in ASCII
$ xpm password-manager|pm generate|g <LENGTH>
# generate password in HEX
$ xpm password-manager|pm generate|g <LENGTH> --hex
# save the generated password
$ xpm password-manager|pm generate|g <LENGTH> --save <NAME>


# Exapmle:
## 33 character in ASCII
$ xpm pm g 33

## 16 character in HEX
$ xpm pm g 16 --hex

## 33 character in ASCII with save
$ xpm pm g 33 --save "github acount"

## 16 character in HEX with save 
$ xpm pm g 16 --hex --save "test xpm v2"
```

### save:
Save custom password in the database.
```sh
# save password
$ xpm password-manager|pm save <NAME>

# Exapmle:
$ xpm pm save "My custom password"
## It will ask you to enter the password.
## NOTE: If password manager database is encrypted it will ask you to enter the decryption key.
```

### find:
Search for password in the database.
```sh
# find password
$ xpm password-manager|pm find <STRING>

# Exapmle:
$ xpm pm find "github"
## NOTE: If password manager database is encrypted it will ask you to enter the decryption key.
## It will search base on the string to find the password that contains the string in the password name.
## NOTE: It is not case sensitive.
## It will display all passwords that match the string in this format:
###     <ID> - <CREATE-AT> - <UPDATE-AT> - <NAME>: <PASSWORD>
```

### show:
Display all passwords in the database.
```sh
# display passwords
## NOTE: this will display all passwords in this format:
###     <ID> - <CREATE-AT> - <UPDATE-AT> - <NAME>: <PASSWORD>
$ xpm password-manager|pm show
# display passwords in table format
## NOTE: this will not show the password, it will show only:
###     id, name, create_at, update_at as table.
$ xpm password-manager|pm show --table|-t

# Exapmle:
$ xpm pm show
## NOTE: If password manager database is encrypted it will ask you to enter the decryption key.
## OUTPUT: 1 - 2025-04-08 15:32:12 - 2025-04-10 15:32:12 - API token: 7FD47C5238D41635AB22E35B09FB6D

$ xpm pm show --table
## NOTE: If password manager database is encrypted it will ask you to enter the decryption key.
## OUTPUT:
###     ╭────┬──────────────────────────────┬─────────────────────┬─────────────────────╮
###     │ id │             name             │      create_at      │      update_at      │
###     ├────┼──────────────────────────────┼─────────────────────┼─────────────────────┤
###     │ 1  │ XPManager GitHub             │ 2025-04-08 15:32:12 │ 2025-04-09 08:43:46 │
###     │ 2  │ API token                    │ 2025-04-08 15:32:12 │ 2025-04-08 15:32:12 │
###     │ 3  │ test xpm                     │ 2025-04-09 08:38:55 │ 2025-04-09 08:38:55 │
###     ╰────┴──────────────────────────────┴─────────────────────┴─────────────────────╯
```

### count:
Get the number of passwords you saved in the database.
```sh
# count passwords
$ xpm password-manager|pm count

# Exapmle:
$ xpm pm count
## NOTE: If password manager database is encrypted it will ask you to enter the decryption key.
## OUTPUT: [INFO] - [count-password] there is 3 in the database. - 4ms
```

### update:
Update the password information.
```sh
# update password
# NOTE: <ID> is integer.
$ xpm password-manager|pm update|up <ID> --password|-p
# update the password name
$ xpm password-manager|pm update|up <ID> --name|-n
# update both password and its name
$ xpm password-manager|pm update|up <ID> --name|-n --password|-n

# Exapmle:
## Update the password with id 1
$ xpm pm update 1 --password
## NOTE: If password manager database is encrypted it will ask you to enter the decryption key.
## It will ask you to enter the password.
## OUTPUT: [INFO] - [update-password] there is 1 password updated successfully. - 15ms

$ xpm pm update 1 --password --name
## NOTE: If password manager database is encrypted it will ask you to enter the decryption key.
## It will ask you to enter both password and name.
## OUTPUT: [INFO] - [update-password] there is 1 password updated successfully. - 15ms
```

### delete:
Delete specific password from the database.
```sh
# delete password
# NOTE: <ID> is integer.
$ xpm password-manager|pm delete <ID>

# Exapmle:
## Delete the password with id 1
$ xpm pm delete 1
## NOTE: If password manager database is encrypted it will ask you to enter the decryption key.
## OUTPUT: [INFO] - [delete-password] there is 1 deleted. - 12ms
```

### encrypt:
Encrypt the password manager database.
```sh
# encrypt passwords manager database without key
$ xpm password-manager|pm encrypt
# encrypt passwords manager database with key
$ xpm password-manager|pm encrypt --key

# Exapmle:
## encrypt without key
$ xpm pm encrypt
## NOTE: If password manager database is encrypted it will not encrypt it once again.
## It will display to you the key.
## Please store the key somewhere safe. 
## WARNING: if you lose the key, you will not be able to recover the passwords.

## encrypt with key
$ xpm pm encrypt --key
## NOTE: If password manager database is encrypted it will not encrypt it once again.
## It will ask you to enter the key.
```

### decrypt:
Decrypt the password manager database.
```sh
# decrypt passwords manager database
$ xpm password-manager|pm decrypt

# Exapmle:
## decrypt
$ xpm pm decrypt
## NOTE: If password manager database is decrypted it will not decrypt it once again.
## It will ask you to enter the key.
```

## encryption-manager:
Encrypt/Decrypt file and folder and encode/decode strings.

### encrypt-file:
Encrypt file.
```sh
# NOTE: --delete will wipe and delete the origin file
# encrypt file without key or delete
$ xpm encryption-manager|em encrypt-file|enf <PATH>
# encrypt file with key
$ xpm encryption-manager|em encrypt-file|enf <PATH> --key
# encrypt file with key and delete
$ xpm encryption-manager|em encrypt-file|enf <PATH> --key --delete
# encrypt file without key and with delete
$ xpm encryption-manager|em encrypt-file|enf <PATH> --delete

# Example:
## encrypt file without key or delete
$ xpm em enf "~/myfolder/myfile.txt"
## It will display to you the key.
## Please store the key somewhere safe. 
## WARNING: if you lose the key, you will not be able to recover the file.

## encrypt file with key
$ xpm em enf "~/myfolder/myfile.txt" --key
## It will ask you to enter the key.

## encrypt file with key and delete
$ xpm em enf "~/myfolder/myfile.txt" --key --delete
## It will ask you to enter the key.
## WARNING: It will wipe and detele the origin file.
###     in this case the origin file is 'myfile.txt'

## encrypt file without key and with delete
$ xpm em enf "~/myfolder/myfile.txt" --delete
## It will display to you the key.
## WARNING: It will wipe and detele the origin file.
###     in this case the origin file is 'myfile.txt'
```

### decrypt-file:
Decrypt file.
```sh
# NOTE: --delete will wipe and delete the origin file.
# NOTE: --xpmv1 use to decrypt XPManager v1.0 files.
# WARNING: Do NOT use --xpmv1 with files has been encrypted using XPManager >= v2.0
# decrypt file without delete
$ xpm encryption-manager|em decrypt-file|def <PATH>
# decrypt file with delete
$ xpm encryption-manager|em decrypt-file|def <PATH> --delete
# decrypt file that has been encrypted using XPManager v1.0 without delete
$ xpm encryption-manager|em decrypt-file|def <PATH> --xpmv1
# decrypt file that has been encrypted using XPManager v1.0 without delete
$ xpm encryption-manager|em decrypt-file|def <PATH> --xpmv1 --delete

# Example:
## decrypt file without delete
$ xpm em def "~/myfolder/myfile.txt.x"
## It will ask you to enter the key.

## decrypt file with delete
$ xpm em def "~/myfolder/myfile.txt.x" --delete
## It will ask you to enter the key.
## WARNING: It will wipe and detele the origin file.
###     in this case the origin file is 'myfile.txt.x'

## decrypt file that has been encrypted using XPManager v1.0 without delete
$ xpm em def "~/myfolder/myfile.txt.x" --xpmv1
## NOTE: You need to decode the old v1.0 key using 'xpm em decode --xpmv1'
## It will ask you to enter the key.

## decrypt file that has been encrypted using XPManager v1.0 without delete
$ xpm em def "~/myfolder/myfile.txt.x" --xpmv1 --delete
## NOTE: You need to decode the old v1.0 key using 'xpm em decode --xpmv1'
## It will ask you to enter the key.
## WARNING: It will wipe and detele the origin file.
###     in this case the origin file is 'myfile.txt.x'
```

### encrypt-dir:
Encrypt directory.
```sh
# NOTE: --delete will wipe and delete the origin files.
# encrypt folder without key or delete
$ xpm encryption-manager|em encrypt-dir|end <PATH>
# encrypt folder without key and with delete
$ xpm encryption-manager|em encrypt-dir|end <PATH> --delete
# encrypt folder with key and without delete
$ xpm encryption-manager|em encrypt-dir|end <PATH> --key
# encrypt folder with key and delete
$ xpm encryption-manager|em encrypt-dir|end <PATH> --key --delete

# Example:
## encrypt folder without key or delete
$ xpm em end "~/myfolder"
## It will ask you to confirm the process.
## It will display to you the key.

# encrypt folder without key and with delete
$ xpm xpm em end "~/myfolder" --delete
## It will ask you to confirm the process.
## It will display to you the key.
## WARNING: It will wipe and detele the origin files.
###     in this case the origin files is any file has been encrypted in '~/myfolder'

# encrypt folder with key and without delete
$ xpm em end "~/myfolder" --key
## It will ask you to enter the key.
## It will ask you to confirm the process.

# encrypt folder with key and delete
$ xpm em end "~/myfolder" --key --delete
## It will ask you to enter the key.
## It will ask you to confirm the process.
## WARNING: It will wipe and detele the origin files.
###     in this case the origin files is any file has been encrypted in '~/myfolder'
```

### decrypt-dir:
Decrypt directory.
```sh
# NOTE: --delete will wipe and delete the origin files.
# NOTE: --xpmv1 use to decrypt XPManager v1.0 folders.
# WARNING: Do NOT use --xpmv1 with folder has been encrypted using XPManager >= v2.0
# decrypt folder without delete
$ xpm encryption-manager|em decrypt-dir|ded <PATH>
# decrypt folder with delete
$ xpm encryption-manager|em decrypt-dir|ded <PATH> --delete
# decrypt folder that has been encrypted using XPManager v1.0 without delete
$ xpm encryption-manager|em decrypt-dir|ded <PATH> --xpmv1
# decrypt folder that has been encrypted using XPManager v1.0 with delete
$ xpm encryption-manager|em decrypt-dir|ded <PATH> --xpmv1 --delete

# Example
## decrypt folder without delete
$ xpm em ded "~/myfolder"
## It will ask you to enter the key.

## decrypt folder with delete
$ xpm em ded "~/myfolder" --delete
## It will ask you to enter the key.
## It will ask you to confirm the process.
## WARNING: It will wipe and detele the origin files.
###     in this case the origin files is any file has been deccrypted in '~/myfolder'

## decrypt folder that has been encrypted using XPManager v1.0 without delete
$ xpm em ded "~/myfolder" --xpmv1
## NOTE: You need to decode the old v1.0 key using 'xpm em decode --xpmv1'
## It will ask you to enter the key.

## decrypt folder that has been encrypted using XPManager v1.0 with delete
$ xpm em ded "~/myfolder" --xpmv1 --delete
## NOTE: You need to decode the old v1.0 key using 'xpm em decode --xpmv1'
## It will ask you to enter the key.
## It will ask you to confirm the process.
## WARNING: It will wipe and detele the origin files.
###     in this case the origin files is any file has been deccrypted in '~/myfolder'
```

### encode:
Encode strings using different techniques.
```sh
# encode strings as hexadecimal
$ xpm encryption-manager|em encode|enc 
# encode strings as hexadecimal 
$ xpm encryption-manager|em encode|enc --hex
# encode strings as binary 
$ xpm encryption-manager|em encode|enc --bin
# encode strings as XPManager v1.0 key 
$ xpm encryption-manager|em encode|enc --xpmv1
# encode strings as hexadecimal hash
$ xpm encryption-manager|em encode|enc --hex-hash

# Example:
## encode strings as hexadecimal
$ xpm em enc
## It will ask you to enter the string
## It will display the encode
## INPUT: XPManager
## OUTPUT: 58 50 4D 61 6E 61 67 65 72

## encode strings as hexadecimal
$ xpm em enc --hex
## It will ask you to enter the string
## It will display the encode
## INPUT: xpm
## OUTPUT: 78 70 6D

## encode strings as binary 
$ xpm em enc --bin
## It will ask you to enter the string
## It will display the encode
## INPUT: xpm
## OUTPUT: 1111000 1110000 1101101

## encode strings as XPManager v1.0 key 
$ xpm em enc --xpmv1
## It will ask you to enter the string
## It will ask you to enter the constant
## It will display the encode
## INPUT: 
###     string: xpm
###     constant: 9999
## OUTPUT: 0x124f08%$%0x111690%$%0x10a163

## encode strings as hex hash
$ xpm em enc --hex-hash
## It will ask you to enter the string
## It will display the encode
## INPUT: Rust is the best
## OUTPUT: 1AE DC 141 1AE
```


### decode:
Decode strings using different techniques.
```sh
# decode strings as hexadecimal
$ xpm encryption-manager|em decode|dec
# decode strings as hexadecimal 
$ xpm encryption-manager|em decode|dec --hex
# decode strings as binary 
$ xpm encryption-manager|em decode|dec --bin
# decode strings as XPManager v1.0 key 
$ xpm encryption-manager|em decode|dec --xpmv1

# Example:
## decode strings as hexadecimal
$ xpm em dec
## It will ask you to enter the encode string 
## It will display the decode
## INPUT: 58 50 4D 61 6E 61 67 65 72
## OUTPUT: XPManager

## decode strings as hexadecimal
$ xpm em dec --hex
## It will ask you to enter the encode string
## It will display the decode
## INPUT: 78 70 6D
## OUTPUT: xpm

## decode strings as binary 
$ xpm em dec --bin
## It will ask you to enter the string
## It will display the decode
## INPUT: xpm
## OUTPUT: 1111000 1110000 1101101

## decode strings as XPManager v1.0 key 
$ xpm em dec --xpmv1
## It will ask you to enter the encode string
## It will ask you to enter the constant
## It will display the decode
## INPUT: 
###     encode: 0x124f08%$%0x111690%$%0x10a163
###     constant: 9999
## OUTPUT: xpm
```

## backup-manager:
Create/Restore backup for passwords/logs database.

### backup:
Create backup for passwords/logs database.
```sh
# backup the password manager database
$ xpm backup-manager|bm backup <SAVE-PATH> --password 
# backup the log manager database
$ xpm backup-manager|bm backup <SAVE-PATH> --log 
# backup the log manager and password manager database
$ xpm backup-manager|bm backup <SAVE-PATH> --log --password 

# Example:
## backup the password manager database
$ xpm bm backup "~/myfolder/backup_folder" --password
## NOTE: It will not create the backup if the password manager database is not encrypted.
## In this case you will find the backup at '~/myfolder/backup_folder/passwords.db.x'

## backup the log manager database
$ xpm bm backup "~/myfolder/backup_folder" --log 
## In this case you will find the backup at '~/myfolder/backup_folder/xpm-log.db'

## backup the log manager and password manager database
$ xpm bm backup "~/myfolder/backup_folder" --log --password 
## NOTE: It will not create the backup if the password manager database is not encrypted.
## In this case you will find the log backup at '~/myfolder/backup_folder/xpm-log.db'
## And the passwords backup at '~/myfolder/backup_folder/passwords.db.x'
```

### restore:
Restore passwords/logs database.
```sh
# restore the password manager database
$ xpm backup-manager|bm restore <PATH> --password 
# restore the log manager database
$ xpm backup-manager|bm restore <PATH> --log
# restore the XPManager v1.0 passwords database
$ xpm backup-manager|bm restore <PATH> --xpmv1
# restore passwords from a json file
## NOET: We support the key value fomrat like this:
###     { 
###         "<PASSWORD-NAME>": "<PASSWORD>"
###     }
$ xpm backup-manager|bm restore <PATH> --password-json

# Example:
## restore the password manager database
$ xpm bm restore "~/myfolder/backup_folder/passwords.db.x" --password
## It will ask you to enter the decryption key to decrypt the restore file
## NOTE: If password manager database is encrypted it will ask you to enter the decryption key.
## NOTE: It will do merge and add the passwords above your passwords.

## restore the log manager database
$ xpm bm restore "~/myfolder/backup_folder/xpm-log.db" --log
## WARNING: If the restore file is encrypted we can not restore it
##      If you encrypt it by yourself, decrypt it and then restore it.
## NOTE: We do not support the merge in the log manager restore,
##      so it will delete the old database and restore the new one.

## restore the XPManager v1.0 passwords database
$ xpm bm restore "~/myfolder/backup_folder/.passwords.json.x" --xpmv1
## It will ask you to enter the decryption key to decrypt the restore file
## NOTE: You need to decode the old v1.0 key using 'xpm em decode --xpmv1'
## NOTE: If password manager database is encrypted it will ask you to enter the decryption key.
## NOTE: It will do merge and add the passwords above your passwords.

## restore passwords from a json file
$ xpm bm restore "~/myfolder/backup_folder/custom-passwords.json" --password-json
## WARNING: If the restore file is encrypted we can not restore it
##      If you encrypt it by yourself, decrypt it and then restore it.
## NOTE: It will do merge and add the passwords above your passwords.
```

## log-manager:
Show, find, delete, or clear xpm logs.

### clear:
Clear all logs.
```sh
# clear XPManager log database
$ xpm log-manager|lm clear

# Example:
## clear XPManager log database
$ xpm lm clear
## WARNING: This will delete all logs.
```

### show:
Display all/some logs.
```sh
# show all logs
$ xpm log-manager|lm show
# show some logs
$ xpm log-manager|lm show --length|-l <LENGTH>

# Example:
## show all logs
$ xpm lm show
## OUTPUT:
###     ╭─────┬─────────────────────────────────────────────────────┬─────────────────────╮
###     │ id  │                        log                          │      create_at      │
###     ├─────┼─────────────────────────────────────────────────────┼─────────────────────┤
###     │  1  │ file '~/myfolder/passwords.db' wiped                │ 2025-04-10 06:38:18 │
###     │  2  │ encrypt file at '~/myfolder/passwords.db'           │ 2025-04-10 07:06:06 │
###     │  3  │ file '~/myfolder/passwords.db' encrypted            │ 2025-04-10 07:06:31 │
###     │  4  │ password with id 2 updated                          │ 2025-04-10 10:59:58 │
###     │  5  │ delete password with id 3: rows affected 1          │ 2025-04-10 11:07:30 │
###     ╰─────┴─────────────────────────────────────────────────────┴─────────────────────╯

## show some logs
$ xpm lm show -l 2
###     ╭─────┬─────────────────────────────────────────────────────┬─────────────────────╮
###     │ id  │                        log                          │      create_at      │
###     ├─────┼─────────────────────────────────────────────────────┼─────────────────────┤
###     │  1  │ file '~/myfolder/passwords.db' wiped                │ 2025-04-10 06:38:18 │
###     │  2  │ encrypt file at '~/myfolder/passwords.db'           │ 2025-04-10 07:06:06 │
###     ╰─────┴─────────────────────────────────────────────────────┴─────────────────────╯
```

### find:
Find logs based on date/string.
```sh
# find logs based on year
$ xpm log-manager|lm find --year <YEAR>
# find logs based on month
$ xpm log-manager|lm find --month <MONTH>
# find logs based on day
$ xpm log-manager|lm find --day <DAY>
# find logs based on date
$ xpm log-manager|lm find --year <YEAR> --month <MONTH> --day <DAY>
# find using string
$ xpm log-manager|lm find <STRING>

# Example:
## find logs based on year
$ xpm lm find --year 2025
## It will display any log created at 2025

## find logs based on month
$ xpm lm find --month 4
## It will display any log created at 4

## find logs based on day
$ xpm lm find --day 10
## It will display any log created at 10

## find logs based on date
$ xpm lm find --year 2025 --month 4 --day 10
## It will display any log created at 2025 04 10

## find using string
$ xpm lm find "passwords"
## It will display any log has string "passwords"
```

### delete:
Delete single log by id.
```sh
# delete log based on id
$ xpm log-manager|lm delete <ID>

# Example:
## delete log based on id
$ xpm lm delete 2
## OUTPUT: [INFO] - [delete-log] deleted 1 from the database. - 11ms
```