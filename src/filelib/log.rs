use super::{
    loglib,
    data_dir,
    errorlib,
    PathBuf
};

pub fn get_log_db_path() -> PathBuf {
    let logger = loglib::Logger::new("get-log-db-pth");
    if let Some(path) = data_dir() {
        return path.join(
            "XPManager/data/xpm-log.db"
        );
    } else {
        logger.error(
            "can NOT get the system data directory path!", 
            errorlib::ExitErrorCode::SystemDataDirNotFound
        );
    }
}