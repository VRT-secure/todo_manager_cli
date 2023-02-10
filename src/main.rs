mod logic_of_program;
use logic_of_program::{choice, create_or_connect_to_db};

fn main() {
    let table_name = "todo_list";
    let column_name = "task";
    let conn = create_or_connect_to_db(table_name, column_name);
    choice(&conn, table_name, column_name);
}
