// PLANNING:
// 1. Read command line arguments: update, upload or help.
// 2a. Show help info if args are strange or empty.
// 2b. Start update function if args is "update", same for upload and help.

// UPLOAD FUNCTION:
// 1. Show rules for uploading, such as making sure all missions are complete and all crafts are in stable orbits/locations.
// 2. Ask for discord username to label changes. (Possibly check if username is member on Fusion Discord.)
// 3a. Ask for path to save file. e.g. "/Users/home/Library/Application Support/Steam/steamapps/common/Spaceflight Simulator/SpaceflightSimulatorGame.app/Saving/Worlds/Testing"
// 3b. Sanitize path input (turning " " to "\ ", etc), and make sure it is a valid path and save file.
// 4. Figure out what crafts have been added, destroyed or altered.
// 5. Append discord username to added crafts in step 4.
// 6. Get current master save from GitHub.
// 7. Check for edge cases and resolve (Look at EDGE CASES).
// 8. Merge remaining new crafts.

// UPDATE FUNCTION:
// 1a. Ask for path to save file. e.g. "/Users/home/Library/Application Support/Steam/steamapps/common/Spaceflight Simulator/SpaceflightSimulatorGame.app/Saving/Worlds/Testing"
// 1b. Sanitize path input (turning " " to "\ ", etc), and make sure it is a valid path and save file.
// Download latest save file from GitHub and move to game's save folder.

// EDGE CASES:
// • 2 people work on space station/base collaboratively, each starting with save containing the first module. If the first person adds a module, and the second person adds a different module, how do you combine the different crafts?

use std::io;

fn main() {
    println!("\nWelcome to Fusion Industry's pseudo-multiplayer shared save file!\nDeveloped by pixelgaming579 using Rust and GitHub.");
    main_menu();
}

fn get_command(input: String) {
    match input.as_str() {
        "quit\n" => {},
        "help\n" => {main_menu();},
        "rules\n" => {rules_and_info();},
        "update\n" => {update();},
        "upload\n" => {upload();},
        // For Windows, which use \r\n instead of \n.
        "quit\r\n" => {},
        "help\r\n" => {main_menu();},
        "rules\r\n" => {rules_and_info();},
        "update\r\n" => {update();},
        "upload\r\n" => {upload();},
        _ => {println!("Invalid input!"); main_menu();}
    }
}

fn main_menu() {
    println!(" • If you are new to the shared save file, make sure you read the uploading rules and info by typing \"rules\".");
    println!(" • To download the latest shared save file, type \"update\".");
    println!(" • To upload your crafts to the shared save file, type \"upload\".");
    println!(" • To quit this program, type \"quit\".");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read user input!");
    get_command(input);
}

fn rules_and_info() {
    println!("\nThese rules are in place to make sure everyone that uses the shared save file has fun. Please read these before you use the system.
Violation of these rules may result in the banning of your Discord account from the shared save file system.");

    println!("\nRules:");
    println!(" 1. Do not name or build crafts inappropriately. If it's not allowed in the Fusion discord, it won't be allowed here.");
    println!(" 2. Do not add to, destroy, or alter someone else's craft without permisson.");
    println!(" 3. Try to keep part counts to a minimum, since everyone that uses this has to download the save file.");
    println!(" 4. Don't flood the launchpad or space with random crafts/junk (especially LEO). We don't want to have a Kessler syndrome situation.");
    println!(" 5. Avoid placing crafts where they could be annoying for other users.");
    println!(" 6. Anything in #rules of the Fusion discord that could apply here will be enforced.");

    println!("\nExtra Info:");
    println!(" • When updating, your local shared save file will be overwritten. Please ensure that no changes you would like to keep haven't been uploaded before updating your local save.");
    println!(" • When uploading, all in-game saves (quicksaves) will be deleted to ensure the save menu doesn't get clogged up.");
    println!(" • Changes will be summarised and sent as a Fusion Bot Discord message in #!!!INSERT-CHANNEL-NAME!!! to ensure players know if something abnormal happens.");
    
    println!("\nIf you want to give feedback or believe something incorrect has happened,
such as abnormal craft deletion or bad moderation, ping pixelgaming579 in the Fusion discord.\n");
    
    main_menu();
}

fn update() {
    println!("UPDATING!")
}

fn upload() {
    println!("UPLOADING!")
}