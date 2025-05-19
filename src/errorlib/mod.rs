
pub enum ExitErrorCode {
    // File
    FileNotFound           = 50,
    FileCreate             = 51,
    FileOpen               = 52,
    FileRead               = 53,
    FileWrite              = 54,
    FileSeek               = 55,
    FileFlush              = 56,
    FileDelete             = 57,
    FileAlreadyEncrypted   = 58,
    FileNotEncrypted       = 59,

    // Dir
    DirNotFound            = 65,
    DirCreate              = 66,
    DirUnsupported         = 67,
    CanNotGetDirData       = 68,
    SystemDataDirNotFound  = 69,
    CanNotGetFileOrDirType = 70,

    // JSON
    CanNotGetJsonObject    = 75,
    InvalidJson            = 76,

    // Encryption and Decryption
    InvalidKey             = 80,
    InvalidEncryptionData  = 81,

    // Database
    DBConnection           = 85,
    DBInsert               = 86,
    DBCreateTable          = 87,
    PMDatabaseNotFound     = 89,
    PMDatabaseEmpty        = 90,
    LMDatabaseNotFound     = 91,
    LogNotFound            = 92,
    LMDatabaseEncrypted    = 93,

    // Others
    Input                  = 95,
    MissingArg             = 96,
    ConfirmationNotMatch   = 97,
    SampleContainSpace     = 98,
}