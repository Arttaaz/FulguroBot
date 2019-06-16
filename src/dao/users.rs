use rusqlite::types::ToSql;
use super::open_db;

pub fn add_user(id: i64, name: String, nb_coq: i64) {
    let conn = open_db();

    conn.execute("INSERT INTO USERS (id, name, nb_coq) VALUES ?1, ?2, ?3", &[&id as &dyn ToSql, &name as &dyn ToSql, &nb_coq as &dyn ToSql]).expect("Failed to add user");
    conn.close().unwrap();
}

pub fn user_exists(id: i64) -> bool {
    let conn = open_db();

    if let Err(_) = conn.query_row("SELECT id FROM USERS WHERE id = ?", &[&id as &dyn ToSql], |row| Ok(row.column_count())) {
        return false;
    }
    true
}

pub fn add_coq_to_user(id: i64, nb_coq: i64) {
    let conn = open_db();

    let pred_coq: i64 = conn.query_row("SELECT nb_coq FROM USERS WHERE id = ?", &[&id as &dyn ToSql], |row| row.get(0)).expect("Could not retrieve nb_coq");
    conn.execute("UPDATE USERS SET nb_coq = ?1 WHERE id = ?2", &[&(pred_coq + nb_coq) as &dyn ToSql, &id as &dyn ToSql]).expect("Failed to change nb_coq");
    conn.close().unwrap();
}

pub fn _delete_user(id: i64) {
    let conn = open_db();

    conn.execute("DELETE FROM USERS WHERE id = ?", &[&id as &dyn ToSql]).expect("Could not delete user");
    conn.close().unwrap();
}
