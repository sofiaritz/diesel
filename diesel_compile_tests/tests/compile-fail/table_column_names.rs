#[macro_use] extern crate diesel;

table! {
    users {
        id -> Integer,
        users -> Integer,
    }
}
// error-pattern: Column `users` cannot be named the same as its table.
// error-pattern: You may use `#[sql_name = "users"]` to reference the table's `users` column.
// error-pattern: Docs available at: `https://docs.diesel.rs/diesel/macro.table.html`

fn main() {}
