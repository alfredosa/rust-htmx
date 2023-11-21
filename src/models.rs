use diesel::{
    ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable,
    SelectableHelper,
};

use crate::schema::users;

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

impl User {
    pub fn create(
        conn: &mut PgConnection,
        firstname: &str,
        lastname: &str,
        email: &str,
        password: &str,
    ) -> Result<User, diesel::result::Error> {
        let new_user = NewUser {
            firstname,
            lastname,
            email,
            password,
        };

        diesel::insert_into(users::table)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(conn)
    }

    pub fn find_by_email(conn: &mut PgConnection, email: &str) -> Option<User> {
        users::dsl::users
            .select(User::as_select())
            .filter(users::dsl::email.eq(email))
            .first(conn)
            .ok()
    }
}
