use clap::{
    Args, 
    Parser, 
    Subcommand
};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct RustflixArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create, update, delete or show users
    User(UserCommand),

    /// Create, update, delete or show Files
    File(FileCommand),
}

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub command: UserSubcommand,
    
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    /// Create a new user
    Create(CreateUser),

    /// Update an existing user
    Update(UpdateUser),

    /// Delete a user
    Delete(DeleteEntity),

    /// Show all users
    Show,
}

#[derive(Debug, Args)]
pub struct CreateUser {
    /// The name of the user
    pub username: String,

    /// The password_hash of the user
    pub password_hash: String,

    /// The password_hash of the user
    pub phone: String,
}

#[derive(Debug, Args)]
pub struct UpdateUser {
    /// The id of the user to update
    pub id: i32,

    /// The name of the user
    pub username: String,

    /// The password_hash of the user
    pub password_hash: String,

    /// The phone of the user
    pub phone: String,
}

#[derive(Debug, Args)]
pub struct DeleteEntity {
    /// The id of the entity to delete
    pub id: i32,
}

#[derive(Debug, Args)]
pub struct FileCommand {
    #[clap(subcommand)]
    pub command: FileSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum FileSubcommand {
    /// Create a new File
    Create(CreateFile),

    /// Update an existing File
    Update(UpdateFile),

    /// Delete a File
    Delete(DeleteEntity),

    /// Show all Files
    Show,
}

#[derive(Debug, Args)]
pub struct CreateFile {
    /// The filename of the File to create
    pub filename: String,

    /// The file_data of the File to create
    pub file_data: Vec<u8>,
}

#[derive(Debug, Args)]
pub struct UpdateFile {
    /// The id of the File to update
    pub id: i32,

    /// The filename of the File
    pub filename: String,

    /// The file_data of the File
    pub file_data: Vec<u8>,
}

