use std::{
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

use crate::dat;

// Q: Why not just move the entire folder (dimensions/minecraft/overworld/data) instead of each individual file?
// A: Minecraft 26.1 doesn't move any files that aren't specific to vanilla Minecraft.
//    "DIM-1/data/capabilities.dat" will not get transferred over because that's a modded file.
//    So the existing folders may still exist, and you can't merge folders just by moving "folder1" to "folder2"'s location.

struct WorldFileInfo {
    // /home/watduhhekbro/downloads/galarov/GALAROV_TEST_done/dimensions/minecraft/overworld/data/minecraft/chunk_tickets.dat
    absolute_path: PathBuf,
    // /home/watduhhekbro/downloads/galarov/GALAROV_TEST_done
    //world_path: PathBuf,
    // /home/watduhhekbro/downloads/galarov/GALAROV_TEST_done (Downgraded)
    //new_world_path: PathBuf,
    // dimensions/minecraft/overworld/data/minecraft/chunk_tickets.dat
    relative_path: PathBuf,
    // data/chunks.dat
    transformed_path: PathBuf,
    nbt_operation: Option<i32>,
}

enum TransformType {
    SimpleMove(PathBuf),
    //SimpleMove(String),
    MoveAndNbtEdit(PathBuf, i32),
    KeepAsIs,
}

const ERROR_PATH_STRIP_PREFIX: &str =
    "path.strip_prefix() didn't work even with identical prefixes";

pub fn create_downgraded_copy(world_path: &String) {
    let world_path = Path::new(&world_path);
    let new_world_path = {
        let mut new_file_name = world_path.file_name().unwrap().to_os_string();
        new_file_name.push(" (Downgraded)");

        //let world_path_str = world_path.to_str().unwrap();
        //let new_world_path_str = format!("{world_path_str} (Downgraded)");
        world_path.with_file_name(new_file_name)
    };

    if fs::exists(&new_world_path).unwrap() {
        println!("\"{}\" already exists!", new_world_path.display());
        return;
    }

    // First: Gather all move operations
    let operations = get_list_of_all_move_operations(world_path).unwrap();

    // Then: Actually do the moving
    let result = execute_file_operations(&new_world_path, &operations);

    // If unsuccessful, then delete new directory
    if let Err(error) = result {
        eprintln!(
            "{error}\n\nCleaning up {}... (currently you need to manually delete it)",
            new_world_path.display()
        );
        //fs::remove_dir_all(new_world_path).unwrap();
    }
}

fn get_list_of_all_move_operations(
    world_path: &Path,
) -> Result<Vec<WorldFileInfo>, walkdir::Error> {
    let mut operations = Vec::new();

    for entry in WalkDir::new(world_path) {
        let entry = entry?;
        if entry.file_type().is_dir() {
            continue;
        }

        let absolute_path = entry.into_path();
        let relative_path = absolute_path
            .strip_prefix(world_path)
            .expect(ERROR_PATH_STRIP_PREFIX);
        let transformed_path = match get_transformed_path_dirs(relative_path) {
            TransformType::SimpleMove(path_buf) => path_buf.to_path_buf(),
            TransformType::MoveAndNbtEdit(path_buf, _) => path_buf.to_path_buf(),
            // If there's nothing to do, you still copy the file over, just at the same location.
            TransformType::KeepAsIs => relative_path.to_path_buf(),
        };
        let info = WorldFileInfo {
            absolute_path: absolute_path.clone(),
            //world_path: world_path.to_path_buf(),
            //new_world_path: world_path.join(" (Downgraded)"),
            relative_path: relative_path.to_path_buf(),
            transformed_path,
            nbt_operation: None,
        };
        operations.push(info);
    }

    Ok(operations)
}

fn get_transformed_path_dirs(relative_path: &Path) -> TransformType {
    // NOTE: This could probably be done better... but oh well, no modded dimension support yet

    // -----

    /*let mut components = relative_path.components();

    if let Some(Component::Normal(dir)) = components.next() {
        // Dimension Data
        if dir == "dimensions" {
            let has_mod_id = components.next().is_some();
            let has_dimension_id = components.next().is_some();
            let data_dir = components.next();

            if has_mod_id && has_dimension_id && let Some(data_dir) = data_dir {
                if data_dir == Component::Normal(OsStr::new("data")) {
                    if components.next().is_some() {
                        //
                    }
                }
                TransformType::SimpleMove(Box::new(Path::new("").join("")))
            } else {
                TransformType::KeepAsIs
            }
        } else {
            TransformType::KeepAsIs
        }
    } else {
        TransformType::KeepAsIs
    }*/

    // -----

    // Specific dimension data files (comes before general dimension handlers below)
    /*if relative_path == "dimensions/minecraft/overworld/data/minecraft/raids.dat" {
        return TransformType::SimpleMove(String::new());
    }
    if relative_path == "" {
        return TransformType::SimpleMove(String::new());
    }
    if relative_path == "" {
        return TransformType::SimpleMove(String::new());
    }

    // General dimension data (entities, poi, region)
    if let Some(relative_path) = relative_path.strip_prefix("dimensions/minecraft/overworld") {
        return TransformType::SimpleMove(String::from(relative_path));
    }
    if let Some(relative_path) = relative_path.strip_prefix("dimensions/minecraft/the_nether") {
        return TransformType::SimpleMove(format!("DIM-1/{relative_path}"));
    }
    if let Some(relative_path) = relative_path.strip_prefix("dimensions/minecraft/the_end") {
        return TransformType::SimpleMove(Box::new(Path::new("DIM1").join(relative_path)));
    }

    // Player Data
    if let Some(relative_path) = relative_path.strip_prefix("players/advancements") {
        return TransformType::SimpleMove(Box::new(Path::new("advancements").join(relative_path)));
    }
    if let Some(relative_path) = relative_path.strip_prefix("players/data") {
        return TransformType::SimpleMove(Box::new(Path::new("playerdata").join(relative_path)));
    }
    if let Some(relative_path) = relative_path.strip_prefix("players/stats") {
        return TransformType::SimpleMove(Box::new(Path::new("stats").join(relative_path)));
    }

    TransformType::KeepAsIs*/

    // -----

    // Dimension Data
    /*if relative_path.starts_with("dimensions") {
        //
    }

    // Specific dimension data files (comes before general dimension handlers below)
    if relative_path == "" {
        return TransformType::SimpleMove(Box::new(Path::new("").join(relative_path)));
    }*/

    // NBT Stuff
    if relative_path == "level.dat" {
        dat::asdf();
        return TransformType::KeepAsIs;
    }

    // Specific One-Offs
    if relative_path == "data/minecraft/maps/last_id.dat" {
        return TransformType::SimpleMove(PathBuf::from("data/idcounts.dat"));
    }
    if relative_path == "resourcepacks/resources.zip" {
        return TransformType::SimpleMove(PathBuf::from("resources.zip"));
    }
    if let Ok(path) = relative_path.strip_prefix("generated/namespace/structure") {
        return TransformType::SimpleMove(Path::new("generated/namespace/structures").join(path));
    }
    /*if relative_path == "data/minecraft/random_sequences.dat" {
        return TransformType::SimpleMove(PathBuf::from("data/random_sequences.dat"));
    }
    if relative_path == "data/minecraft/scoreboard.dat" {
        return TransformType::SimpleMove(PathBuf::from("data/scoreboard.dat"));
    }
    if relative_path == "data/minecraft/stopwatches.dat" {
        return TransformType::SimpleMove(PathBuf::from("data/stopwatches.dat"));
    }*/

    // Maps
    if let Ok(path) = relative_path.strip_prefix("data/minecraft/maps") {
        let new_file_name = path.file_name().unwrap().to_str().unwrap();
        return TransformType::SimpleMove(Path::new("data").join(format!("map_{new_file_name}")));
    }

    // General Data Namespace
    if let Ok(path) = relative_path.strip_prefix("data/minecraft") {
        return TransformType::SimpleMove(Path::new("data").join(path));
    }

    // General dimension data (entities, poi, region)
    if let Ok(path) = relative_path.strip_prefix("dimensions/minecraft/overworld") {
        // <world>/data/raids.dat = <world>/dimensions/minecraft/overworld/data/minecraft/raids.dat
        if let Ok(path) = path.strip_prefix("data/minecraft") {
            if path == "chunk_tickets.dat" {
                return TransformType::SimpleMove(Path::new("data").join("chunks.dat"));
            } else {
                return TransformType::SimpleMove(Path::new("data").join(path));
            }
        }
        // <world>/region = <world>/dimensions/minecraft/overworld/region
        else {
            return TransformType::SimpleMove(Path::new("").join(path));
        }
    }
    if let Ok(path) = relative_path.strip_prefix("dimensions/minecraft/the_nether") {
        // <world>/data/raids.dat = <world>/dimensions/minecraft/overworld/data/minecraft/raids.dat
        if let Ok(path) = path.strip_prefix("data/minecraft") {
            if path == "chunk_tickets.dat" {
                return TransformType::SimpleMove(Path::new("DIM-1/data").join("chunks.dat"));
            } else {
                return TransformType::SimpleMove(Path::new("DIM-1/data").join(path));
            }
        }
        // <world>/region = <world>/dimensions/minecraft/overworld/region
        else {
            return TransformType::SimpleMove(Path::new("DIM-1").join(path));
        }
    }
    if let Ok(path) = relative_path.strip_prefix("dimensions/minecraft/the_end") {
        // <world>/data/raids.dat = <world>/dimensions/minecraft/overworld/data/minecraft/raids.dat
        if let Ok(path) = path.strip_prefix("data/minecraft") {
            if path == "chunk_tickets.dat" {
                return TransformType::SimpleMove(Path::new("DIM1/data").join("chunks.dat"));
            } else if path == "raids.dat" {
                return TransformType::SimpleMove(Path::new("DIM1/data").join("raids_end.dat"));
            } else {
                return TransformType::SimpleMove(Path::new("DIM1/data").join(path));
            }
        }
        // <world>/region = <world>/dimensions/minecraft/overworld/region
        else {
            return TransformType::SimpleMove(Path::new("DIM1").join(path));
        }
    }

    // Player Data
    if let Ok(path) = relative_path.strip_prefix("players/advancements") {
        return TransformType::SimpleMove(Path::new("advancements").join(path));
    }
    if let Ok(path) = relative_path.strip_prefix("players/data") {
        return TransformType::SimpleMove(Path::new("playerdata").join(path));
    }
    if let Ok(path) = relative_path.strip_prefix("players/stats") {
        return TransformType::SimpleMove(Path::new("stats").join(path));
    }

    TransformType::KeepAsIs
}

fn execute_file_operations(
    new_world_path: &Path,
    operations: &Vec<WorldFileInfo>,
) -> Result<(), std::io::Error> {
    for action in operations {
        // Create all parent folders
        fs::create_dir_all(new_world_path.join(&action.transformed_path.parent().expect("All")))?;

        // Move each file
        fs::copy(
            &action.absolute_path,
            new_world_path.join(&action.transformed_path),
        )?;
    }

    Ok(())
}
