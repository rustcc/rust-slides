use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::{
    tasks,
    tasks::dsl::{completed as task_completed, tasks as all_tasks},
};

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Insertable,  Serialize, Deserialize)]
#[table_name = "tasks"]
pub struct NewTask {
    #[serde(rename = "Desc")]
    pub description: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub completed: bool,
}


impl Task {
    pub fn all(conn: &PgConnection) -> QueryResult<Vec<Task>> {
        all_tasks.order(tasks::id.desc()).load::<Task>(conn)
    }

    pub fn insert(todo: NewTask, conn: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(tasks::table)
            .values(&todo)
            .execute(conn)
    }
}