use self::models::*;
use diesel::prelude::*;
use diesel_demo::*;
use webhook_poc::establish_connection;

fn main() {
    add_user();
}

fn add_user() {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .limit(5)
        .select(Users::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
