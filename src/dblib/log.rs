use super::{
    Connection,
    PathBuf,
    Tabled
};
use crate::{
    loglib,
    filelib,
    errorlib
};

#[derive(Tabled)]
pub struct LogInfoForamt {
    pub id: u32,
    pub log: String,
    pub create_at: String
}

fn create_log_table(log_db_path: PathBuf) {
    let logger = loglib::Logger::new("create-logs-table");
    if let Ok(conn) = Connection::open(&log_db_path) {
        if let Err(_) = conn.execute("
            CREATE TABLE IF NOT EXISTS logs(
                id INTEGER PRIMARY KEY,
                log TEXT NOT NULL,
                create_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
        ", []) {
            conn.close().unwrap();
            filelib::delete_file(log_db_path);
            logger.error(
                "can NOT create the logs table!", 
                errorlib::ExitErrorCode::DBCreateTable
            );
        }
        logger.info("logs table created successfully.");
    } else {
        logger.error(
            &format!(
                "can NOT create connection with '{}'", 
                log_db_path.display()
            ),
            errorlib::ExitErrorCode::DBConnection
        );
    }
}

pub fn register(log: &str) {
    let logger = loglib::Logger::new("register-log");
    let log_db_path = filelib::log::get_log_db_path();
    let log_db_state = filelib::get_file_state(
        log_db_path.to_str().unwrap().to_string()
    );
    if log_db_state == filelib::FileState::NotFound {
        filelib::create_file(log_db_path.clone());
        create_log_table(log_db_path.clone());
    } else if log_db_state == filelib::FileState::Decrypted {
        if let Ok(conn) = Connection::open(&log_db_path) {
            if let Err(_) = conn.execute("INSERT INTO logs (log) VALUES (?1)", [log]) {
                conn.close().unwrap();
                filelib::delete_file(log_db_path);
                logger.error(
                    "can NOT insert into the logs table!", 
                    errorlib::ExitErrorCode::DBInsert
                );
            }
        } else {
            logger.error(
                &format!(
                    "can NOT create connection with '{}'", 
                    log_db_path.display()
                ),
                errorlib::ExitErrorCode::DBConnection
            );
        }
    }
}

pub fn delete_all(log_db_path: PathBuf) -> usize {
    let logger = loglib::Logger::new("delete-all-logs");
    if let Ok(conn) = Connection::open(&log_db_path) {
        return conn
            .execute("DELETE FROM logs", [])
            .unwrap_or(0);
    } else {
        logger.error(
            &format!(
                "can NOT create connection with '{}'", 
                log_db_path.display()
            ),
            errorlib::ExitErrorCode::DBConnection
        );
    }
}

pub fn get_logs(log_db_path: PathBuf, length: u16, string: String) -> Vec<LogInfoForamt> {
    let logger = loglib::Logger::new("get-logs");
    if let Ok(conn) = Connection::open(&log_db_path) {
        let sql = if length > 0 {
            &format!("
                    SELECT id, log, create_at 
                    FROM logs
                    ORDER BY create_at DESC, id DESC
                    LIMIT {}
                ", 
                length
            )
        } else if string.len() > 0 {
            &format!("
                    SELECT id, log, create_at 
                    FROM logs
                    WHERE log LIKE '%{}%'
                ", 
                string
            )
        } else {
            "SELECT id, log, create_at FROM logs"
        };
        let e = conn.prepare(sql);
        if let Ok(mut stmt) = e {
            let password: Result<Vec<LogInfoForamt>, _> = stmt.query_map([], |row| {
                Ok(
                    LogInfoForamt {
                        id: row.get::<_, u32>(0).unwrap(),
                        log: row.get::<_, String>(1).unwrap(),
                        create_at: row.get::<_, String>(2).unwrap(),
                    }
                )
            })
                .unwrap()
                .collect();
            return password.unwrap();
        }
    }
    logger.error(
        &format!(
            "can NOT create connection with '{}'", 
            log_db_path.display()
        ),
        errorlib::ExitErrorCode::DBConnection
    );
}

pub fn get_logs_by_date(log_db_path: PathBuf, date: (u16, u8, u8)) -> Vec<LogInfoForamt> {
    let logger = loglib::Logger::new("get-logs");
    if let Ok(conn) = Connection::open(&log_db_path) {
        let mut sql: String = "
            SELECT id, log, create_at
            FROM logs
            WHERE log LIKE '%%'
        ".to_owned();
        // SELECT * FROM logs
        // WHER log LIKE '%%' -- to use the AND date
        //      AND strftime('%Y', create_at) = year
        //      AND strftime('%m', create_at) = month
        //      AND strftime('%d', create_at) = day
        if date.0 > 0 {
            sql += &format!(" AND strftime('%Y', create_at) = '{:04}'", date.0);
        }
        if date.1 > 0 {
            sql += &format!(" AND strftime('%m', create_at) = '{:02}'", date.1);
        }
        if date.2 > 0 {
            sql += &format!(" AND strftime('%d', create_at) = '{:02}'", date.2);
        }
        let e = conn.prepare(&sql);
        if let Ok(mut stmt) = e {
            let password = stmt.query_map(
                [],
                |row| {
                    Ok(
                        LogInfoForamt {
                            id: row.get::<_, u32>(0).unwrap(),
                            log: row.get::<_, String>(1).unwrap(),
                            create_at: row.get::<_, String>(2).unwrap(),
                        }
                    )
            })
                .unwrap()
                .collect::<Result<Vec<_>, _>>();
            return password.unwrap();
        }
    }
    logger.error(
        &format!(
            "can NOT create connection with '{}'", 
            log_db_path.display()
        ),
        errorlib::ExitErrorCode::DBConnection
    );
}

pub fn delete_one(log_db_path: PathBuf, id: String) -> usize {
    let logger = loglib::Logger::new("delete-log");
    if let Ok(conn) = Connection::open(&log_db_path) {
        return conn.execute("
                DELETE FROM logs 
                WHERE id=?1
            ",
            [id]
        ).unwrap_or(0);
    } else {
        logger.error(
            &format!(
                "can NOT create connection with '{}'", 
                log_db_path.display()
            ),
            errorlib::ExitErrorCode::DBConnection
        );
    }
}