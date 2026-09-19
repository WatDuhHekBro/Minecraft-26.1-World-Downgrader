# Minecraft 26.1 World Downgrader

A small tool to downgrade worlds in the new level format introduced in Minecraft 26.1 to the old 1.21.11 format.

...better late than never, right?

*Manual downgrades beyond that point (like opening the world in 1.21.1) should work fine-ish. Probably.*

## TODO

- level.dat
    - New splinter files
- Miscellaneous Stuff

## Known Issues

- Doesn't support modded dimensions
    - Would need figure out `modid` to `DIM#` conversion first (or prompt?)
    - `chunks.dat`, `raids.dat`, `world_border.dat`
- The `level.dat` conversion might miss some more obscure fields, as the old Wiki documentation (pre 26.1) was noticeably outdated
    - This program errs on the side of caution and adds duplicate NBT entries on level.dat. You can use an NBT editor like [Dovetail](https://offroaders123.github.io/Dovetail/) to edit out these extra fields.

## Personal Observations

Going from 26.1.2 to 1.21.11 incurs no apparent data loss. However, going from 1.21.11 to 1.21.1, there are some things I notice:
- Specifically armor you are wearing will disappear
- Signs will become blank

If I use [Chunker](https://www.chunker.app/) to convert the 26.1.2 chunks directly to 1.21.1 and manually merge the result with my downgrader, what happens?
- 26.3 to 26.2 requires it, or you will spawn in a void world for some reason.
