# Wiki Reference

https://minecraft.wiki/w/Java_Edition_26.1



# Full Renames

```
/data/map_#.dat = /data/minecraft/maps/#.dat
data/idcounts.dat -> data/minecraft/maps/last_id.dat
data/random_sequences.dat -> data/minecraft/random_sequences.dat
        renamed:    data/scoreboard.dat -> data/minecraft/scoreboard.dat
        renamed:    data/stopwatches.dat -> data/minecraft/stopwatches.dat

<world>/data/chunks.dat = <world>/dimensions/minecraft/overworld/data/minecraft/chunk_tickets.dat
<world>/data/raids.dat = <world>/dimensions/minecraft/overworld/data/minecraft/raids.dat
<world>/data/world_border.dat = <world>/dimensions/minecraft/overworld/data/minecraft/world_border.dat
DIM1/data/raids_end.dat

<world>/data = <world>/dimensions/minecraft/overworld/data
<world>/entities = <world>/dimensions/minecraft/overworld/entities
<world>/poi = <world>/dimensions/minecraft/overworld/poi
<world>/region = <world>/dimensions/minecraft/overworld/region

<world>/DIM-1/data = <world>/dimensions/minecraft/the_nether/data
<world>/DIM-1/entities = <world>/dimensions/minecraft/the_nether/entities
<world>/DIM-1/poi = <world>/dimensions/minecraft/the_nether/poi
<world>/DIM-1/region = <world>/dimensions/minecraft/the_nether/region

<world>/DIM1/data = <world>/dimensions/minecraft/the_end/data
<world>/DIM1/entities = <world>/dimensions/minecraft/the_end/entities
<world>/DIM1/poi = <world>/dimensions/minecraft/the_end/poi
<world>/DIM1/region = <world>/dimensions/minecraft/the_end/region

<world>/advancements/*.json = <world>/players/advancements/*.json
<world>/playerdata/*.dat(_old) = <world>/players/data/*.dat(_old)
<world>/stats/*.json = <world>/players/stats/*.json
```



# Miscellaneous

```
deleted:    data/Mansion_index.dat
deleted:    data/Mineshaft_index.dat
deleted:    data/Monument_index.dat
deleted:    data/Stronghold_index.dat
deleted:    data/Temple_index.dat
deleted:    data/Village_index.dat
new file:   data/minecraft/custom_boss_events.dat
new file:   data/minecraft/game_rules.dat
new file:   data/minecraft/scheduled_events.dat
new file:   data/minecraft/wandering_trader.dat
new file:   data/minecraft/weather.dat
new file:   data/minecraft/world_clocks.dat
new file:   data/minecraft/world_gen_settings.dat
new file:   dimensions/minecraft/the_end/data/minecraft/ender_dragon_fight.dat
modified:   level.dat
deleted:    level.dat_old
```



# git

```
[main aa23b60] 26.1
 189 files changed, 0 insertions(+), 0 deletions(-)
 delete mode 100644 data/Mansion_index.dat
 delete mode 100644 data/Mineshaft_index.dat
 delete mode 100644 data/Monument_index.dat
 delete mode 100644 data/Stronghold_index.dat
 delete mode 100644 data/Temple_index.dat
 delete mode 100644 data/Village_index.dat
 create mode 100644 data/minecraft/custom_boss_events.dat
 create mode 100644 data/minecraft/game_rules.dat
 rename data/{map_0.dat => minecraft/maps/0.dat} (100%)
 rename data/{map_1.dat => minecraft/maps/1.dat} (100%)
 rename data/{map_10.dat => minecraft/maps/10.dat} (100%)
 rename data/{map_11.dat => minecraft/maps/11.dat} (100%)
 rename data/{map_12.dat => minecraft/maps/12.dat} (100%)
 rename data/{map_13.dat => minecraft/maps/13.dat} (100%)
 rename data/{map_14.dat => minecraft/maps/14.dat} (100%)
 rename data/{map_15.dat => minecraft/maps/15.dat} (100%)
 rename data/{map_16.dat => minecraft/maps/16.dat} (100%)
 rename data/{map_17.dat => minecraft/maps/17.dat} (100%)
 rename data/{map_18.dat => minecraft/maps/18.dat} (100%)
 rename data/{map_19.dat => minecraft/maps/19.dat} (100%)
 rename data/{map_2.dat => minecraft/maps/2.dat} (100%)
 rename data/{map_20.dat => minecraft/maps/20.dat} (100%)
 rename data/{map_21.dat => minecraft/maps/21.dat} (100%)
 rename data/{map_22.dat => minecraft/maps/22.dat} (100%)
 rename data/{map_23.dat => minecraft/maps/23.dat} (100%)
 rename data/{map_24.dat => minecraft/maps/24.dat} (100%)
 rename data/{map_25.dat => minecraft/maps/25.dat} (100%)
 rename data/{map_26.dat => minecraft/maps/26.dat} (100%)
 rename data/{map_27.dat => minecraft/maps/27.dat} (100%)
 rename data/{map_28.dat => minecraft/maps/28.dat} (100%)
 rename data/{map_29.dat => minecraft/maps/29.dat} (100%)
 rename data/{map_3.dat => minecraft/maps/3.dat} (100%)
 rename data/{map_30.dat => minecraft/maps/30.dat} (100%)
 rename data/{map_31.dat => minecraft/maps/31.dat} (100%)
 rename data/{map_32.dat => minecraft/maps/32.dat} (100%)
 rename data/{map_33.dat => minecraft/maps/33.dat} (100%)
 rename data/{map_34.dat => minecraft/maps/34.dat} (100%)
 rename data/{map_35.dat => minecraft/maps/35.dat} (100%)
 rename data/{map_36.dat => minecraft/maps/36.dat} (100%)
 rename data/{map_37.dat => minecraft/maps/37.dat} (100%)
 rename data/{map_38.dat => minecraft/maps/38.dat} (100%)
 rename data/{map_39.dat => minecraft/maps/39.dat} (100%)
 rename data/{map_4.dat => minecraft/maps/4.dat} (100%)
 rename data/{map_40.dat => minecraft/maps/40.dat} (100%)
 rename data/{map_41.dat => minecraft/maps/41.dat} (100%)
 rename data/{map_42.dat => minecraft/maps/42.dat} (100%)
 rename data/{map_43.dat => minecraft/maps/43.dat} (100%)
 rename data/{map_44.dat => minecraft/maps/44.dat} (100%)
 rename data/{map_45.dat => minecraft/maps/45.dat} (100%)
 rename data/{map_46.dat => minecraft/maps/46.dat} (100%)
 rename data/{map_47.dat => minecraft/maps/47.dat} (100%)
 rename data/{map_48.dat => minecraft/maps/48.dat} (100%)
 rename data/{map_49.dat => minecraft/maps/49.dat} (100%)
 rename data/{map_5.dat => minecraft/maps/5.dat} (100%)
 rename data/{map_50.dat => minecraft/maps/50.dat} (100%)
 rename data/{map_51.dat => minecraft/maps/51.dat} (100%)
 rename data/{map_52.dat => minecraft/maps/52.dat} (100%)
 rename data/{map_53.dat => minecraft/maps/53.dat} (100%)
 rename data/{map_54.dat => minecraft/maps/54.dat} (100%)
 rename data/{map_55.dat => minecraft/maps/55.dat} (100%)
 rename data/{map_56.dat => minecraft/maps/56.dat} (100%)
 rename data/{map_57.dat => minecraft/maps/57.dat} (100%)
 rename data/{map_58.dat => minecraft/maps/58.dat} (100%)
 rename data/{map_59.dat => minecraft/maps/59.dat} (100%)
 rename data/{map_6.dat => minecraft/maps/6.dat} (100%)
 rename data/{map_60.dat => minecraft/maps/60.dat} (100%)
 rename data/{map_61.dat => minecraft/maps/61.dat} (100%)
 rename data/{map_62.dat => minecraft/maps/62.dat} (100%)
 rename data/{map_63.dat => minecraft/maps/63.dat} (100%)
 rename data/{map_7.dat => minecraft/maps/7.dat} (100%)
 rename data/{map_8.dat => minecraft/maps/8.dat} (100%)
 rename data/{map_9.dat => minecraft/maps/9.dat} (100%)
 rename data/{idcounts.dat => minecraft/maps/last_id.dat} (100%)
 rename data/{ => minecraft}/random_sequences.dat (100%)
 create mode 100644 data/minecraft/scheduled_events.dat
 rename data/{ => minecraft}/scoreboard.dat (100%)
 rename data/{ => minecraft}/stopwatches.dat (100%)
 create mode 100644 data/minecraft/wandering_trader.dat
 create mode 100644 data/minecraft/weather.dat
 create mode 100644 data/minecraft/world_clocks.dat
 create mode 100644 data/minecraft/world_gen_settings.dat
 rename DIM-1/data/chunks.dat => dimensions/minecraft/overworld/data/minecraft/chunk_tickets.dat (100%)
 rename {DIM-1/data => dimensions/minecraft/overworld/data/minecraft}/raids.dat (100%)
 rename {DIM-1/data => dimensions/minecraft/overworld/data/minecraft}/world_border.dat (100%)
 rename {entities => dimensions/minecraft/overworld/entities}/r.-1.-1.mca (100%)
 rename {entities => dimensions/minecraft/overworld/entities}/r.-1.0.mca (100%)
 rename {entities => dimensions/minecraft/overworld/entities}/r.-4.-1.mca (100%)
 rename {entities => dimensions/minecraft/overworld/entities}/r.-5.-2.mca (100%)
 rename {entities => dimensions/minecraft/overworld/entities}/r.-5.-3.mca (100%)
 rename {entities => dimensions/minecraft/overworld/entities}/r.0.-1.mca (100%)
 rename {entities => dimensions/minecraft/overworld/entities}/r.0.0.mca (100%)
 rename {entities => dimensions/minecraft/overworld/entities}/r.1.-4.mca (100%)
 rename {entities => dimensions/minecraft/overworld/entities}/r.2.0.mca (100%)
 rename {poi => dimensions/minecraft/overworld/poi}/r.-1.-1.mca (100%)
 rename {poi => dimensions/minecraft/overworld/poi}/r.-1.0.mca (100%)
 rename {poi => dimensions/minecraft/overworld/poi}/r.-1.1.mca (100%)
 rename {poi => dimensions/minecraft/overworld/poi}/r.-4.-1.mca (100%)
 rename {poi => dimensions/minecraft/overworld/poi}/r.-5.-2.mca (100%)
 rename {poi => dimensions/minecraft/overworld/poi}/r.0.-1.mca (100%)
 rename {poi => dimensions/minecraft/overworld/poi}/r.0.0.mca (100%)
 rename {poi => dimensions/minecraft/overworld/poi}/r.0.1.mca (100%)
 rename {poi => dimensions/minecraft/overworld/poi}/r.1.-4.mca (100%)
 rename {poi => dimensions/minecraft/overworld/poi}/r.2.0.mca (100%)
 rename {region => dimensions/minecraft/overworld/region}/r.-1.-1.mca (100%)
 rename {region => dimensions/minecraft/overworld/region}/r.-1.0.mca (100%)
 rename {region => dimensions/minecraft/overworld/region}/r.-1.1.mca (100%)
 rename {region => dimensions/minecraft/overworld/region}/r.-3.-4.mca (100%)
 rename {region => dimensions/minecraft/overworld/region}/r.-4.-1.mca (100%)
 rename {region => dimensions/minecraft/overworld/region}/r.-4.-4.mca (100%)
 rename {region => dimensions/minecraft/overworld/region}/r.-5.-2.mca (100%)
 rename {region => dimensions/minecraft/overworld/region}/r.-5.-3.mca (100%)
 rename {region => dimensions/minecraft/overworld/region}/r.0.-1.mca (100%)
 rename {region => dimensions/minecraft/overworld/region}/r.0.0.mca (100%)
 rename {region => dimensions/minecraft/overworld/region}/r.0.1.mca (100%)
 rename {region => dimensions/minecraft/overworld/region}/r.1.-4.mca (100%)
 rename {region => dimensions/minecraft/overworld/region}/r.2.0.mca (100%)
 rename DIM1/data/chunks.dat => dimensions/minecraft/the_end/data/minecraft/chunk_tickets.dat (100%)
 create mode 100644 dimensions/minecraft/the_end/data/minecraft/ender_dragon_fight.dat
 rename {data => dimensions/minecraft/the_end/data/minecraft}/raids.dat (100%)
 rename {DIM1/data => dimensions/minecraft/the_end/data/minecraft}/world_border.dat (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/entities/r.-1.-1.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/entities/r.-1.-3.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/entities/r.-1.0.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/entities/r.0.-1.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/entities/r.0.0.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/entities/r.1.-3.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/entities/r.2.-4.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/entities/r.3.-4.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/poi/r.-1.-1.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/poi/r.-1.-3.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/poi/r.-1.0.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/poi/r.0.-1.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/poi/r.0.-2.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/poi/r.0.-3.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/poi/r.0.-4.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/poi/r.0.0.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/poi/r.1.-3.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/poi/r.2.-3.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/poi/r.2.-4.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/poi/r.3.-4.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/region/r.-1.-1.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/region/r.-1.-3.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/region/r.-1.0.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/region/r.0.-1.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/region/r.0.-2.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/region/r.0.-3.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/region/r.0.-4.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/region/r.0.0.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/region/r.1.-3.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/region/r.2.-3.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/region/r.2.-4.mca (100%)
 rename {DIM1 => dimensions/minecraft/the_end}/region/r.3.-4.mca (100%)
 rename data/chunks.dat => dimensions/minecraft/the_nether/data/minecraft/chunk_tickets.dat (100%)
 rename DIM1/data/raids_end.dat => dimensions/minecraft/the_nether/data/minecraft/raids.dat (100%)
 rename {data => dimensions/minecraft/the_nether/data/minecraft}/world_border.dat (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/entities/r.-1.-1.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/entities/r.-1.0.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/entities/r.-1.1.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/entities/r.0.-1.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/entities/r.0.0.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/entities/r.0.1.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/entities/r.0.2.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/entities/r.1.1.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/entities/r.1.2.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/poi/r.-1.0.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/poi/r.-1.1.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/poi/r.-1.2.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/poi/r.0.-1.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/poi/r.0.0.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/poi/r.0.1.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/poi/r.0.2.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/poi/r.1.1.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/poi/r.1.2.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/region/r.-1.-1.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/region/r.-1.0.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/region/r.-1.1.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/region/r.-1.2.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/region/r.0.-1.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/region/r.0.0.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/region/r.0.1.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/region/r.0.2.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/region/r.1.1.mca (100%)
 rename {DIM-1 => dimensions/minecraft/the_nether}/region/r.1.2.mca (100%)
 delete mode 100644 level.dat_old
 rename {advancements => players/advancements}/851732b5-ab32-40ae-8b4e-6d8d140b767e.json (100%)
 rename {playerdata => players/data}/851732b5-ab32-40ae-8b4e-6d8d140b767e.dat (100%)
 rename {playerdata => players/data}/851732b5-ab32-40ae-8b4e-6d8d140b767e.dat_old (100%)
 rename {stats => players/stats}/851732b5-ab32-40ae-8b4e-6d8d140b767e.json (100%)
```

```
[main aa23b60] 26.1
 189 files changed, 0 insertions(+), 0 deletions(-)
 delete mode 100644 data/Mansion_index.dat
 delete mode 100644 data/Mineshaft_index.dat
 delete mode 100644 data/Monument_index.dat
 delete mode 100644 data/Stronghold_index.dat
 delete mode 100644 data/Temple_index.dat
 delete mode 100644 data/Village_index.dat
 create mode 100644 data/minecraft/custom_boss_events.dat
 create mode 100644 data/minecraft/game_rules.dat
 create mode 100644 data/minecraft/scheduled_events.dat
 create mode 100644 data/minecraft/wandering_trader.dat
 create mode 100644 data/minecraft/weather.dat
 create mode 100644 data/minecraft/world_clocks.dat
 create mode 100644 data/minecraft/world_gen_settings.dat
 create mode 100644 dimensions/minecraft/the_end/data/minecraft/ender_dragon_fight.dat
 delete mode 100644 level.dat_old
```



# Path (as of MC 26.1.2)

- `net/minecraft/util/filefix/FileFixerUpper.java::fix()` "Starting upgrade for world"
- Leads into `this.startOrContinueFileFixing()`
- Schemas located at `net/minecraft/util/filefix/fixes/*.java::makeFixer()`

# Operations Dump

1. LegacyStructureFileFix (no clue what it does)
2. ~~ResourcePackLocationFileFix~~
3. ~~DimensionStorageFileFix~~
4. ~~PlayerStorageFileFix~~
5. LevelDatToSavedDataFileFix
6. RemoveObsoleteFilesFileFix (removes stuff like `Mansion_index.dat`, no clue what those do)
7. ~~GeneratedStructuresRenameFileFix~~



# TODO

```
FileFixOperations.moveRegex("command_storage_([a-z0-9_.-]+)\\.dat", "$1/command_storage\\.dat"),
```



# Wiki Dump (Excluded)

```
[Double] BorderCenterX: Center of the world border on the X coordinate. Defaults to 0.
[Double] BorderCenterZ: Center of the world border on the Z coordinate. Defaults to 0.
[Double] BorderDamagePerBlock: Defaults to 0.2.
[Double] BorderSize: Width and length of the border of the world. Defaults to 60000000.
[Double] BorderSafeZone: Defaults to 5.
[Double] BorderSizeLerpTarget: Defaults to 60000000.
[Long] BorderSizeLerpTime: Defaults to 0.
[Double] BorderWarningBlocks: Defaults to 5.
[Double] BorderWarningTime: Defaults to 15.

[NBT Compound / JSON Object] DimensionData: This tag contains level data specific to certain dimensions.

    [NBT Compound / JSON Object] 1: Data for the End.
        [NBT Compound / JSON Object] DragonFight: Data for the ender dragon fight. Appears only after the End is entered.
            [NBT Compound / JSON Object] ExitPortalLocation: Location of the End's exit portal that the ender dragon flies to upon its death.
                [Byte] X: The X coordinate of the portal.
                [Byte] Y: The Y coordinate of the portal.
                [Byte] Z: The Z coordinate of the portal.
            [NBT List / JSON Array] Gateways: Contains a list of locations of the end gateway portals that haven't been spawned.
                [Int]: The angle of a future gateway, from 0 to 19. 0 is east of the exit portal, and numbers increase clockwise.
            [Boolean] DragonKilled: 1 or 0 (true/false) - If the dragon is currently alive.
            [Long] DragonUUIDLeast: The least significant bits of the current ender dragon's Universally Unique IDentifier. This is joined with DragonUUIDMost to form the dragon's unique ID.
            [Long] DragonUUIDMost: The most significant bits of the current ender dragon's Universally Unique IDentifier. This is joined with DragonUUIDLeast to form the dragon's unique ID.
            [Boolean] PreviouslyKilled: 1 or 0 (true/false) - If the ender dragon has ever been defeated. Used to determine how much XP is given by dragon and whether to spawn a Dragon Egg.

[NBT List / JSON Array] enabled_features: List of experimental features enabled for this world. Doesn't appear if there are no experimental features enabled.

    [String]: A single experimental feature.

[String] generatorName: Used in 1.15 and below. The name of the generator; default, flat, largeBiomes, amplified, buffet, debug_all_block_states or default_1_1. Not case sensitive, but always written in the case here. The last one can exist only if the file was edited. It is a variation of the default generation. It can also be customized if it is a customized world from before 1.13. In this case the world becomes default if opened using 1.13 or newer.
[NBT Compound / JSON Object] generatorOptions: Used in 1.15 and below. Used in buffet, superflat, and old customized worlds. Format below.
[Int] generatorVersion: Used in 1.15 and below. The version of the level generator. The effects of changing this are unknown, but values other than 0 have been observed.
[Boolean] MapFeatures: 1 or 0 (true/false) - true if the map generator should place structures such as villages, strongholds, and mineshafts. Defaults to 1. Always 1 if the world type is Customized.
[NBT Compound / JSON Object] Player: The state of the singleplayer player. This overrides the <player>.dat file with the same name as the singleplayer player. This is saved by servers only if it already exists, otherwise it is not saved for server worlds. See Player.dat format.
[Long] RandomSeed: The random level seed used to generate consistent terrain.
[Long] SizeOnDisk: The estimated size in bytes of the level. Currently not modified or used by Minecraft, but was previously.
```
