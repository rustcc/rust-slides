use std::ops::Deref;
use crate::task::{NewTask, Task};
use super::{PgPool, PgPooledConnection};

pub fn create_task(todo: String, pool: &PgPool) -> Result<(), &'static str> {
    let new_task = NewTask { description: todo };
    Task::insert(new_task, get_conn(pool)?.deref())
        .map(|_| ())
        .map_err(|_| "Error inserting task")
}

fn get_conn(pool: &PgPool) -> Result<PgPooledConnection, &'static str> {
    pool.get().map_err(|_| "can get connection")
}