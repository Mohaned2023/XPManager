## XPManager:
It's good to have a CLI tool that manages your passwords and lets you control them and quickly create new passwords of various sizes to suit your needs. This is where XPManager comes in to help you manage passwords, and also allows you to work with files/folders and secure them with the **Fernet** encryption.

## v2.1.0, What's New?
- Fix: Correct spelling errors and improve text clarity.
- New: Refactor exit codes, see [exit codes](./ERROR.md).
- New: Set password manager generate length as optional, see [generate length](./USAGE.md#generate).
- Fix: Show logs with length in descending order, see [show with length](./USAGE.md#show-1).

## Installation Instructions:
Follow the steps below to install XPManager on your system based on your operating system.

### Linux:
- Go to the [Release](https://github.com/Mohaned2023/XPManager/releases) Page and download the latest version of XPMaanager for Linux.
- Open a terminal and run the following commands:
```sh
chmod +x xpm
sudo mv xpm /usr/local/bin
```

### Windows:
- Download the latest version of XPManager from the [Release](https://github.com/Mohaned2023/XPManager/releases) Page.
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

## Exit Codes:
See [exit codes](./ERROR.md) documentation.

---
> By [Mohaned Sherhan (Mr.x)](https://github.com/Mohaned2023)