use crate::args::{
    FileSubcommand, 
    FileCommand, 
    CreateFile, 
    UpdateFile, 
};
use crate::db::establish_connection;
use crate::models::{NewFile, File as DBFile};
use diesel::prelude::*;

pub fn handle_file_command(file: FileCommand) {
    let command = file.command;
    match command {
        FileSubcommand::Create(file) => {
            create_file(file);
        }
        FileSubcommand::Update(file) => {
            update_file(file);
        }
        FileSubcommand::Show => {
            show_files();
        }
        FileSubcommand::Delete(_) => todo!(),
    }
}

fn create_file(file: CreateFile) {
    println!("Creating file: {:?}", file);
    use crate::schema::files::dsl::*;

    let new_file = NewFile {
        filename: &file.filename,
        file_data: &file.file_data,
    };

    diesel::insert_into(files)
        .values(&new_file)
        .execute(&mut establish_connection())
        .expect("Error saving new file");
}

fn update_file(file: UpdateFile) {
    println!("Updating file: {:?}", file);
    use crate::schema::files::dsl::*;

    let db_file = DBFile {
        id: file.id,
        filename: file.filename,
        file_data: file.file_data,
    };
    
    diesel::update(files.find(file.id))
        .set(&db_file)
        .execute(&mut establish_connection())
        .expect("Error updating file");
}

fn show_files() {
    use crate::schema::files::dsl::*;

    let mut connection = establish_connection();
    let results = files
        .load::<DBFile>(&mut connection)
        .unwrap();

    println!("Displaying {} files", results.len());
    for file in results {
        println!("{:?}", file);
    }
}