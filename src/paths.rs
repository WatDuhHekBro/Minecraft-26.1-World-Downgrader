use std::{
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

use crate::{
    dat::{
        self, level_new::NewLevelDat, level_old::convert_new_leveldat_to_old_leveldat,
        wandering_trader::NewWanderingTrader, weather::NewWeather,
        world_gen_settings::NewWorldGenSettings,
    },
    util::SimpleError,
};

// Q: Why not just move the entire folder (dimensions/minecraft/overworld/data) instead of each individual file?
// A: Minecraft 26.1 doesn't move any files that aren't specific to vanilla Minecraft.
//    "DIM-1/data/capabilities.dat" will not get transferred over because that's a modded file.
//    So the existing folders may still exist, and you can't merge folders just by moving "folder1" to "folder2"'s location.

struct WorldFileInfo {
    // /home/watduhhekbro/downloads/galarov/GALAROV_TEST_done/dimensions/minecraft/overworld/data/minecraft/chunk_tickets.dat
    absolute_path: PathBuf,
    // data/chunks.dat
    transformed_path: Option<PathBuf>,
}

enum TransformType {
    Move(PathBuf),
    Drop,
    KeepAsIs,
}

#[derive(Default)]
struct NbtTransfer {
    level: Option<NewLevelDat>,
    //ender_dragon_fight: Option<i32>,
    //custom_boss_events: Option<i32>,
    //game_rules: Option<i32>,
    //scheduled_events: Option<i32>,
    wandering_trader: Option<NewWanderingTrader>,
    weather: Option<NewWeather>,
    //world_clocks: Option<i32>,
    world_gen_settings: Option<NewWorldGenSettings>,
}

const ERROR_PATH_STRIP_PREFIX: &str =
    "path.strip_prefix() didn't work even with identical prefixes";

pub fn create_downgraded_copy(world_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut nbt_table = NbtTransfer::default();
    let new_world_path = {
        let mut new_file_name = world_path.file_name().unwrap().to_os_string();
        new_file_name.push(" (Downgraded)");

        //let world_path_str = world_path.to_str().unwrap();
        //let new_world_path_str = format!("{world_path_str} (Downgraded)");
        world_path.with_file_name(new_file_name)
    };

    if fs::exists(&new_world_path)? {
        return Err(Box::new(SimpleError::from(format!(
            "\"{}\" already exists!",
            new_world_path.display()
        ))));
    }

    // First: Gather all move operations
    let operations = get_list_of_all_move_operations(world_path, &mut nbt_table)?;

    // Then: Actually do the moving
    let result = execute_file_operations(&new_world_path, &operations, &nbt_table);

    // If unsuccessful, then delete new directory
    if let Err(error) = result {
        return Err(Box::new(SimpleError::from(format!(
            "{error}\n\nCleaning up {}... (currently you need to manually delete it)",
            new_world_path.display()
        ))));
        //fs::remove_dir_all(new_world_path).unwrap();
    }

    Ok(())
}

fn get_list_of_all_move_operations(
    world_path: &Path,
    nbt_table: &mut NbtTransfer,
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
        let transformed_path =
            match get_transformed_path_dirs(&absolute_path, relative_path, nbt_table) {
                TransformType::Move(path_buf) => Some(path_buf.to_path_buf()),
                TransformType::Drop => None,
                // If there's nothing to do, you still copy the file over, just at the same location.
                TransformType::KeepAsIs => Some(relative_path.to_path_buf()),
            };
        let info = WorldFileInfo {
            absolute_path: absolute_path.clone(),
            transformed_path,
        };
        operations.push(info);
    }

    Ok(operations)
}

fn get_transformed_path_dirs(
    absolute_path: &Path,
    relative_path: &Path,
    nbt_table: &mut NbtTransfer,
) -> TransformType {
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
                TransformType::Move(Box::new(Path::new("").join("")))
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
        return TransformType::Move(String::new());
    }
    if relative_path == "" {
        return TransformType::Move(String::new());
    }
    if relative_path == "" {
        return TransformType::Move(String::new());
    }

    // General dimension data (entities, poi, region)
    if let Some(relative_path) = relative_path.strip_prefix("dimensions/minecraft/overworld") {
        return TransformType::Move(String::from(relative_path));
    }
    if let Some(relative_path) = relative_path.strip_prefix("dimensions/minecraft/the_nether") {
        return TransformType::Move(format!("DIM-1/{relative_path}"));
    }
    if let Some(relative_path) = relative_path.strip_prefix("dimensions/minecraft/the_end") {
        return TransformType::Move(Box::new(Path::new("DIM1").join(relative_path)));
    }

    // Player Data
    if let Some(relative_path) = relative_path.strip_prefix("players/advancements") {
        return TransformType::Move(Box::new(Path::new("advancements").join(relative_path)));
    }
    if let Some(relative_path) = relative_path.strip_prefix("players/data") {
        return TransformType::Move(Box::new(Path::new("playerdata").join(relative_path)));
    }
    if let Some(relative_path) = relative_path.strip_prefix("players/stats") {
        return TransformType::Move(Box::new(Path::new("stats").join(relative_path)));
    }

    TransformType::KeepAsIs*/

    // -----

    // NBT Stuff
    if relative_path == "level.dat" {
        nbt_table.level = Some(dat::read_leveldat(absolute_path));
        return TransformType::KeepAsIs;
    }
    if relative_path == "dimensions/minecraft/the_end/data/minecraft/ender_dragon_fight.dat" {
        return TransformType::Drop;
    }
    if relative_path == "data/minecraft/custom_boss_events.dat" {
        return TransformType::Drop;
    }
    if relative_path == "data/minecraft/game_rules.dat" {
        return TransformType::Drop;
    }
    if relative_path == "data/minecraft/scheduled_events.dat" {
        return TransformType::Drop;
    }
    if relative_path == "data/minecraft/wandering_trader.dat" {
        nbt_table.wandering_trader = Some(dat::read_wandering_trader(absolute_path));
        return TransformType::Drop;
    }
    if relative_path == "data/minecraft/weather.dat" {
        nbt_table.weather = Some(dat::read_weather(absolute_path));
        return TransformType::Drop;
    }
    if relative_path == "data/minecraft/world_clocks.dat" {
        return TransformType::Drop;
    }
    if relative_path == "data/minecraft/world_gen_settings.dat" {
        nbt_table.world_gen_settings = Some(dat::read_worldgen(absolute_path));
        return TransformType::Drop;
    }

    // Specific One-Offs
    if relative_path == "data/minecraft/maps/last_id.dat" {
        return TransformType::Move(PathBuf::from("data/idcounts.dat"));
    }
    if relative_path == "resourcepacks/resources.zip" {
        return TransformType::Move(PathBuf::from("resources.zip"));
    }
    if let Ok(path) = relative_path.strip_prefix("generated/namespace/structure") {
        return TransformType::Move(Path::new("generated/namespace/structures").join(path));
    }

    // Maps
    if let Ok(path) = relative_path.strip_prefix("data/minecraft/maps") {
        let new_file_name = path.file_name().unwrap().to_str().unwrap();
        return TransformType::Move(Path::new("data").join(format!("map_{new_file_name}")));
    }

    // General Data Namespace
    if let Ok(path) = relative_path.strip_prefix("data/minecraft") {
        return TransformType::Move(Path::new("data").join(path));
    }

    // General dimension data (entities, poi, region)
    if let Ok(path) = relative_path.strip_prefix("dimensions/minecraft/overworld") {
        // <world>/data/raids.dat = <world>/dimensions/minecraft/overworld/data/minecraft/raids.dat
        if let Ok(path) = path.strip_prefix("data/minecraft") {
            if path == "chunk_tickets.dat" {
                return TransformType::Move(Path::new("data").join("chunks.dat"));
            } else {
                return TransformType::Move(Path::new("data").join(path));
            }
        }
        // <world>/region = <world>/dimensions/minecraft/overworld/region
        else {
            return TransformType::Move(Path::new("").join(path));
        }
    }
    if let Ok(path) = relative_path.strip_prefix("dimensions/minecraft/the_nether") {
        // <world>/data/raids.dat = <world>/dimensions/minecraft/overworld/data/minecraft/raids.dat
        if let Ok(path) = path.strip_prefix("data/minecraft") {
            if path == "chunk_tickets.dat" {
                return TransformType::Move(Path::new("DIM-1/data").join("chunks.dat"));
            } else {
                return TransformType::Move(Path::new("DIM-1/data").join(path));
            }
        }
        // <world>/region = <world>/dimensions/minecraft/overworld/region
        else {
            return TransformType::Move(Path::new("DIM-1").join(path));
        }
    }
    if let Ok(path) = relative_path.strip_prefix("dimensions/minecraft/the_end") {
        // <world>/data/raids.dat = <world>/dimensions/minecraft/overworld/data/minecraft/raids.dat
        if let Ok(path) = path.strip_prefix("data/minecraft") {
            if path == "chunk_tickets.dat" {
                return TransformType::Move(Path::new("DIM1/data").join("chunks.dat"));
            } else if path == "raids.dat" {
                return TransformType::Move(Path::new("DIM1/data").join("raids_end.dat"));
            } else {
                return TransformType::Move(Path::new("DIM1/data").join(path));
            }
        }
        // <world>/region = <world>/dimensions/minecraft/overworld/region
        else {
            return TransformType::Move(Path::new("DIM1").join(path));
        }
    }

    // Player Data
    if let Ok(path) = relative_path.strip_prefix("players/advancements") {
        return TransformType::Move(Path::new("advancements").join(path));
    }
    if let Ok(path) = relative_path.strip_prefix("players/data") {
        return TransformType::Move(Path::new("playerdata").join(path));
    }
    if let Ok(path) = relative_path.strip_prefix("players/stats") {
        return TransformType::Move(Path::new("stats").join(path));
    }

    TransformType::KeepAsIs
}

fn execute_file_operations(
    new_world_path: &Path,
    operations: &Vec<WorldFileInfo>,
    nbt_table: &NbtTransfer,
) -> Result<(), std::io::Error> {
    for action in operations {
        if let Some(transformed_path) = &action.transformed_path {
            if transformed_path == "level.dat" {
                let oldleveldat = convert_new_leveldat_to_old_leveldat(
                    nbt_table.level.as_ref().unwrap(),
                    nbt_table.wandering_trader.as_ref().unwrap(),
                    nbt_table.weather.as_ref().unwrap(),
                    nbt_table.world_gen_settings.as_ref().unwrap(),
                );
                dat::write_leveldat(&new_world_path.join(transformed_path), oldleveldat);
            } else {
                // Create all parent folders
                fs::create_dir_all(new_world_path.join(transformed_path.parent().expect("All")))?;

                // Move each file
                fs::copy(&action.absolute_path, new_world_path.join(transformed_path))?;
            }
        }
    }

    Ok(())
}
