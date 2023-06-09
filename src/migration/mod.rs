use skytable::{Connection, ddl::{Keymap, KeymapType, Ddl}};

fn create_keyspace(conn: &mut Connection, name: &str) -> Result<(), skytable::error::Error> {
    conn.create_keyspace(name)?;
    Ok(())
}

fn create_table(conn: &mut Connection, keyspace: &str, name: &str) -> Result<(), skytable::error::Error>{

    let session_table = Keymap::new(format!("{keyspace}:{name}"))
        .set_ktype(KeymapType::Str)
        .set_vtype(KeymapType::Binstr);

    conn.create_table(session_table)?;
    Ok(())
}

#[test]
fn run_db_migrations() {
    let mut conn = Connection::new("127.0.0.1", 2003).unwrap();
    
    //create_keyspace(&mut conn, "analytics").unwrap();
    //create_table(&mut conn, "analytics", "user").unwrap();
}