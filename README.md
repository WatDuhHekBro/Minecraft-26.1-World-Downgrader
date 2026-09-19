# Minecraft 26.1 World Downgrader

A small tool to downgrade worlds in the new Minecraft 26.1 level format to the old 1.21.11 format.

...better late than never, right?

*Manual downgrades beyond that point (like opening the world in 1.21.1) should work fine-ish. Probably.*

## TODO

- level.dat
    - Old level.dat
    - New splinter files
- Miscellaneous Stuff
- Code Cleanup (probably)

## Known Issues

- Doesn't support modded dimensions
    - Would need figure out `modid` to `DIM#` conversion first (or prompt?)
    - `chunks.dat`, `raids.dat`, `world_border.dat`
- The `level.dat` conversion might miss some more obscure fields, as the old Wiki documentation (pre 26.1) was noticeably outdated
    - This program errs on the side of caution and adds duplicate NBT entries on level.dat. You can use an NBT editor like [Dovetail](https://offroaders123.github.io/Dovetail/) to edit out these extra fields.
