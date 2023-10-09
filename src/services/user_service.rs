use crate::models::*;
use diesel::prelude::*;
use webhook_poc::establish_connection;

pub fn get_user(id: i32) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let result = users
        .filter(id.eq(id))
        .limit(1)
        .select(User::as_select())
        .load::<User>(connection)?
        .first()
        .ok_or(diesel::result::Error::NotFound)?
        .clone();

    Ok(result)
}

pub fn add_user(user: User) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let result = diesel::insert_into(users)
        .values(user)
        .get_result::<User>(connection)?;

    Ok(result)
}
