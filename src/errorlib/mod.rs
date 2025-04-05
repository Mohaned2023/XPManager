
pub enum ExitErrorCode {
    FileNotFound        = 2,
    UsageError          = 64,
    NoDataAvilable      = 65,
    CannotCreateDir     = 73,
    CannotCreateFile    = 74,
    CannotAccessDataDir = 72,
    DBConnection        = 111,
    CanNotDeleteFile    = 126,
}