use crate::document_list;
use crate::schema::results;
use diesel::prelude::*;
pub fn create_result(conn: &PgConnection, result: &document_list::Result) -> QueryResult<usize> {
    diesel::insert_into(results::table)
        .values(result)
        .execute(conn)
}
