mod ui;
mod database;


use crate::ui::{AppState, build_ui};
use crate::database::{Database, PassSave};



use druid::{WindowDesc, AppLauncher};

fn main() {
    let database = Database::new("dubg.db")
    .expect("Failed to Create a DataBase struct");

    // Create a table if it doesn't exist
    let _ = database.create_table()
    .expect("Failed to create or open the table");

    let pass_saves = database.query_all_passsaves()
    .expect("Failed to get the enteries");

    let _ = database.insert_passsave("Twitter", "Malcom X", "Bumpy Jonhson")
    .expect("Failed to Add Entery");
 

    for pass_save in pass_saves.clone() {
        println!("Found Entery {}", pass_save);
    }
    
    let main_window = WindowDesc::new(build_ui(pass_saves.clone()))
        .title("Hello World!")
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state: AppState = AppState::new(pass_saves);

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");

    // let main_window = WindowDesc::new(build_ui(pass_saves))     
    //     .title("Hello World!")
    //     .window_size((400.0, 400.0));

    // let _launch = AppLauncher::with_window(main_window)
    //     .log_to_console()
    //     .launch(AppState::new(database
    //         .query_all_passsaves()
    //         .expect("Failed to get the enteries")));
        
    // println!("Hello World");

    // let database = Database::new("dubg.db").expect("Failed to create a Database struct");

    // let _ = database.create_table().expect("Failed to create or open the table");

    // let _ = database
    //     .insert_passsave("Facebook", "Jonh White", "test_password")
    //     .expect("Failed to add entry");

    // let passsaves = database
    //     .query_all_passsaves()
    //     .expect("Failed to get the entries");

    // for passsave in passsaves {
    //     println!("Found Entry {}", passsave);
    // }

    // let initial_state = ui::AppState::new(passsaves);
    // AppLauncher::with_window(ui::main_window())
    //     .launch(initial_state)
    //     .expect("Failed to launch application");
}
