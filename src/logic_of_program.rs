use std::{io, env};
use rusqlite::{params, Connection};

fn input() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn show_variants(){
    println!("1) Ввести новую запись");
    println!("2) Удалить запись");
    println!("3) изменить определённую запись");
    println!("4) Вывести все записи");
    println!("5) Найти определённую запись");
    println!("6) Показать это меню");
    println!("7) Выйти из программы");
}

pub fn choice(conn: &Connection, table_name: &str, column_name: &str){
    show_variants();
    loop {
        let s = input();
        if s == "\n".to_string(){
            continue;
        }

        let ch_num = match s.trim().parse::<u8>() {
            Ok(num) => {num},
            Err(error) => {
                println!("Ошибка {}, нужно ввести число от 1 до 6 включительно", error);
                continue;
            },
        };
        match ch_num {
            1 => {
                println!("\nВведите данные:");
                insert_into_table(&conn, table_name, column_name, input().as_str());
            },
            2 => {
                select_from_table(&conn, table_name);
                println!("Введите id записи: ");
                let id = input_id();
                delete_from_table(&conn, table_name, id);
            },
            3 => {
                select_from_table(&conn, table_name);
                println!("Введите id записи: ");
                let id = input_id();
                println!("Введите новую запись");
                let new_task = input();
                update_in_table(&conn, table_name, column_name, id, new_task.as_str());
            },
            4 => {
                select_from_table(&conn, table_name);
            },
            5 => {
                println!("Введите ключивое слово или предложение, по которому нужно искать: ");
                let keyword = input();
                search_record(&conn, table_name, column_name, keyword.as_str());
            },
            6 => {
                show_variants();
            },
            7 => {
                println!("До скорого ;)");
                break;
            },
            _ => {},
        }
    }
}


fn input_id() -> u32{
    loop {
        let id: u32 = match input().trim().parse() {
            Ok(num) => {num},
            Err(error) => {
                println!("Ошибка {}, нужно ввести целое число больше 0", error);
                continue;
                },
        };
        return id;
    }
}

fn insert_into_table(conn: &Connection, table_name: &str, column_name: &str, data: &str) {
    let sql = format!("INSERT INTO {} ({}) VALUES (?1)", table_name, column_name);
    let result = conn.execute(
        &sql,
        params![data],
    );

    match result {
        Ok(_) => println!("Запись добавлена!"),
        Err(error) => println!("Возникла ошибка {}", error),
    }
}

fn delete_from_table(conn: &Connection, table_name: &str, id: u32) {
    let sql = format!("DELETE FROM {} WHERE id = ?", table_name);
    let result = conn.execute(sql.as_str(), params![id]);
    match result {
        Ok(_) => println!("Запись удалена!"),
        Err(error) => println!("Возникла ошибка {}", error),
    }
}

fn select_from_table(conn: &Connection, table_name: &str){
    let sql = format!("SELECT * FROM {}", table_name);
    let mut stmt = conn.prepare(&sql).unwrap();
    let rows = stmt.query_map(params![], |row| {
        Ok((row.get(0).unwrap(), row.get(1).unwrap()))
    }).unwrap();

    for row in rows {
        let (id, task): (u32, String) = row.unwrap();
        println!("id: {:?} task: {:?}", id, task);
    }
}

pub fn create_or_connect_to_db(table_name: &str, column_name: &str) -> Connection{
    let exe_file_path = env::current_exe().unwrap();
    let path = exe_file_path.parent().unwrap().join(format!("{}.sqlite3", table_name));
    println!("Путь к файлу БД: {:?}", path);
    let conn = Connection::open(path).unwrap();

    let sql = format!(
        "CREATE TABLE IF NOT EXISTS {} ( 
        id INTEGER PRIMARY KEY, 
        {} TEXT NOT NULL )", 
        table_name, column_name);
    
    match conn.execute(&sql,params![],) {
        Ok(_) => {println!("База данных подключена!")},
        Err(error) => {println!("Ошибка создания базы данных {}", error)},
    };
    conn
}

fn update_in_table(conn: &Connection, table_name: &str, column_name: &str, id: u32, new_task: &str){
    let sql = format!("UPDATE {} SET {} = ?1 WHERE id = ?2", table_name, column_name);
    match conn.execute(&sql, &[new_task, &id.to_string()]) {
        Ok(_) => {println!("Запись обновлена!")},
        Err(error) => {println!("Возникла ошибка {}", error)},
    };
}

fn search_record(conn: &Connection, table_name: &str, column_name: &str, keyword: &str){
    let sql = format!("SELECT * FROM {} WHERE {} LIKE '%{}%'", table_name, column_name, keyword);
    let mut stmt = conn.prepare(&sql).unwrap();
    let rows = stmt.query_map(params![], |row| {
        Ok((row.get(0).unwrap(), row.get(1).unwrap()))
    }).unwrap();
    
    for row in rows {
        let (id, task): (u32, String) = row.unwrap();
        println!("id: {:?} task: {:?}", id, task);
    }
}
