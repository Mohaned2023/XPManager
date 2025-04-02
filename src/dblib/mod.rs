use crate::loglib;
use crate::filelib;
use rusqlite::params;
use rusqlite::Connection;

pub struct PasswordInfoForm {
    pub id: i32,
    pub name: String,
    pub password: String
}

pub fn save_password(name: String, password: String) {
    let logger = loglib::Logger::new("save-password");
    let mut password_manager_db_path = filelib::get_password_manager_db_path();
    if filelib::is_encrypted(&password_manager_db_path) {
        // TODO: decrypt the db.
        // TODO: encrypt the db.
        // TODO: secure delete the decrypt db.
    }

    if let Ok(conn) = Connection::open(&password_manager_db_path) {
        let _ = conn.execute("CREATE TABLE IF NOT EXISTS passwords(
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            password TEXT NOT NULL
        )", []);
        let _ = conn.execute("INSERT INTO passwords (name, password) VALUES (?1, ?2)", params![name.clone(), password]);
        logger.info(
            &format!("password saved successfully wiht name '{}'", name)
        );
    } else {
        logger.error(
            &format!("can NOT create connection with '{}'", password_manager_db_path.display())
        );
    }
}

pub fn find_password(string: String) -> Vec<PasswordInfoForm> {
    let logger = loglib::Logger::new("dblib");
    let password_manager_db_path = filelib::get_password_manager_db_path();
    if let Ok(conn) = Connection::open(&password_manager_db_path) {
        if let Ok(mut stmt) = conn.prepare("SELECT * FROM passwords WHERE name LIKE ?1") {
            let pattern = format!("%{}%", string);
            let password = stmt.query_map(params![pattern], |row| {
                Ok(
                    PasswordInfoForm {
                        id: row.get::<_, i32>(0).unwrap(),
                        name: row.get::<_, String>(1).unwrap(),
                        password: row.get::<_, String>(2).unwrap(), 
                    }
                )
            })
                .unwrap()
                .collect::<Result<Vec<_>, _>>();
            return password.unwrap();
        }
        logger.error("can NOT create the query!");
        panic!("can NOT create the query!");
    }
    logger.error(
        &format!("can NOT create connection with '{}'", password_manager_db_path.display())
    );
    panic!("can NOT create connection!");
}