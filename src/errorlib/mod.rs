
pub enum ExitErrorCode {
    // File
    FileNotFound           = 2,
    FileCreate             = 3,
    FileOpen               = 4,
    FileRead               = 5,
    FileWrite              = 6,
    FileSeek               = 8,
    FileFlush              = 9,
    FileDelete             = 10,
    FileAlreadyEncrypted   = 11,
    FileNotEncrypted       = 12,

    // Dir
    DirNotFound            = 20,
    DirCreate              = 21,
    DirUnsupported         = 22,
    CanNotGetDirData       = 23,
    SystemDataDirNotFound  = 24,
    CanNotGetFileOrDirType = 25,

    // JSON
    CanNotGetJsonObject    = 30,
    InvalidJson            = 31,

    // Encryption and Decryption
    InvalidKey             = 35,
    InvalidEncryptionData  = 36,

    // Database
    DBConnection           = 40,
    DBInsert               = 41,
    DBCreateTable          = 42,
    PMDatabaseNotFound     = 43,
    PMDatabaseEmpty        = 44,
    LMDatabaseNotFound     = 45,
    LogNotFound            = 46,

    // Others
    Usage                  = 55,
    MissingArg             = 56,
    ConfirmationNotMatch   = 57,
}