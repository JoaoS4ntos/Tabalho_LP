use crate::args::{
    UserSubcommand, 
    UserCommand, 
    CreateUser, 
    UpdateUser, 
    DeleteEntity
};
use crate::db::establish_connection;
use crate::models::{NewUser, User as DBUser};
use diesel::prelude::*;

pub fn handle_user_command(user: UserCommand) {
    let command = user.command;
    match command {
        UserSubcommand::Create(user) => {
            create_user(user);
        }
        UserSubcommand::Update(user) => {
            update_user(user);
        }
        UserSubcommand::Delete(delete_entity) => {
            delete_user(delete_entity);
        }
        UserSubcommand::Show => {
            show_users();
        }

    }
}

fn create_user(user: CreateUser) {
    println!("Creating user: {:?}", user);
    use crate::schema::users::dsl::*;

    let mut connection = establish_connection();
    let new_user = NewUser {
        username: &user.username,
        password_hash: &user.password_hash,
        phone: &user.phone,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut connection)
        .expect("Error saving new user");
}

fn update_user(user: UpdateUser) {
    println!("Updating user: {:?}", user);
    use crate::schema::users::dsl::*;

    let db_user = DBUser {
        id: user.id,
        username: user.username,
        password_hash: user.password_hash,
        phone: user.phone,
       
    };
    
    diesel::update(users.find(user.id))
        .set(&db_user)
        .execute(&mut establish_connection())
        .expect("Error updating user");
}

fn delete_user(user: DeleteEntity) {
    println!("Deleting user: {:?}", user);
    use crate::schema::users::dsl::*;

    diesel::delete(users.find(user.id))
        .execute(&mut establish_connection())
        .expect("Error deleting user");
}

fn show_users() {
    use crate::schema::users::dsl::*;

    let mut connection = establish_connection();
    let results = users
        .load::<DBUser>(&mut connection)
        .unwrap();

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{:?}", user);
    }
}

pub fn get_user_name(user_id: i32) -> Option<String> {
    use crate::schema::users::dsl::*;

    let user = users.find(user_id).first::<DBUser>(&mut establish_connection()).ok()?;
    Some(user.username)
}

pub fn get_hash_password(user_id: i32) -> Option<String> {
    use crate::schema::users::dsl::*;

    let connection = establish_connection();
    let user = users.find(user_id).first::<DBUser>(&mut establish_connection()).ok()?;
    Some(user.password_hash)
}