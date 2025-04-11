## XPManager v2.0:
From **Python** to **Rust** you will see a performance you never dreamed of. 
XPManager is a CLI tool specialized in dealing with passwords management and 
file encryption/decryption. You can create very strong passwords with any 
length. We support from `1` character to `65,535` character. We also support two 
type of password `ASCII` and `HEX`. For example: ASCII (`muvk?nc#La>,IPLp:JMee3M`) 
or HEX (`505B2E877638504F2B76FF2`). We also store saved passwords securely and as 
per the user's wishes. You can choose whether or not to encrypt your saved passwords.
We use a strong and secure encryption algorithm (**Fernet**) to encrypt passwords as 
well as files. We also have a file wipe algorithm to make it difficult or rather 
almost impossible to recover files that have been encrypted with deletion. 

## What's New?
- **Rust** is the best!
- Fast response and excellent performance.
- Download and run on any operating system.
- File encryption and decryption using blocks (buffers).
- Powerful wiping algorithm with four levels.
- Update database from **JSON** to **sqlite**.
- Ability to process large files.
- Completely disconnected from the internet.
- Ability to create different type of passwords.
- Restructuring the command system.

## Installation Instructions:
Follow the steps below to install XPManager on your system based on your operating system.

### Linux:
- Go to the Release Page and download the latest version of XPMaanager for Linux.
- Open a terminal and run the following commands:
```sh
chmod +x xpm
sudo mv xpm /usr/local/bin
```

### Windows:
- Download the latest version of XPManager from the Release Page.
- Extract the contents and move the XPManager folder to your system drive (e.g., C:\XPManager).
- Add the `bin` folder to your system's PATH:
    - Open Environment Variables settings:
        - Press `Win + X`, then select **System**.
        - Click on Advanced system settings > Environment Variables.
    - Under **System variables**, find and select `Path`, then click **Edit**.
    - Add the path to the `XPManager\bin` folder (e.g., C:\XPManager\bin).
    - Click **OK** to save changes.

### Other:
- You need to install **Rust** and **Cargo** in your system.
- Clone the repo:
```sh
$ git clone https://github.com/Mohaned2023/XPManager.git
$ cd XPManager
```
- Build the bin
```sh
$ cargo build --release
```
- Move `XPManager/target/release/xpm` to your `bin` folder.
- Try to run
```sh
$ xpm --version
```

Once this is done, you should be able to run xpm from the command line.

## Usage:
You can use `--help` with any command to print the help message. also see the [usage](./USAGE.md) documentation.

---
> By [Mohaned Sherhan (Mr.x)](https://github.com/Mohaned2023)