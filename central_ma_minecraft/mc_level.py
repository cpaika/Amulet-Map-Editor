"""Create a minimal level.dat for a flat/superflat-style custom world so the
generated region terrain is used and the game does not overwrite it."""
import os
from mc_nbt import (Compound, List, Int, Byte, Long, String, Double, Float,
                    to_gzip, TAG_DOUBLE)


def write_level_dat(world_dir, name, data_version, spawn=(0, 200, 0),
                    border_center=(0, 0)):
    sx, sy, sz = spawn
    # A void/flat generator so chunks we did not author stay empty rather than
    # being filled by normal worldgen (keeps our terrain authoritative).
    flat_settings = Compound({
        "biome": String("minecraft:plains"),
        "lakes": Byte(0),
        "features": Byte(0),
        "layers": List(10, [
            Compound({"block": String("minecraft:air"), "height": Int(1)}),
        ]),
        "structure_overrides": List(8, []),
    })
    gen = Compound({
        "type": String("minecraft:flat"),
        "settings": flat_settings,
    })
    dimensions = Compound({
        "minecraft:overworld": Compound({
            "type": String("minecraft:overworld"),
            "generator": gen,
        })
    })
    world_gen_settings = Compound({
        "seed": Long(0),
        "generate_features": Byte(0),
        "bonus_chest": Byte(0),
        "dimensions": dimensions,
    })
    data = Compound({
        "version": Int(19133),
        "DataVersion": Int(data_version),
        "LevelName": String(name),
        "GameType": Int(1),          # creative
        "Difficulty": Byte(1),
        "allowCommands": Byte(1),
        "Time": Long(0),
        "DayTime": Long(6000),
        "SpawnX": Int(sx), "SpawnY": Int(sy), "SpawnZ": Int(sz),
        "raining": Byte(0), "thundering": Byte(0),
        "hardcore": Byte(0),
        "initialized": Byte(1),
        "WorldGenSettings": world_gen_settings,
        "GameRules": Compound({
            "doDaylightCycle": String("false"),
            "doWeatherCycle": String("false"),
            "doMobSpawning": String("false"),
            "keepInventory": String("true"),
        }),
        "Version": Compound({
            "Id": Int(data_version),
            "Name": String("1.20.4"),
            "Snapshot": Byte(0),
        }),
    })
    root = Compound({"Data": data})
    os.makedirs(world_dir, exist_ok=True)
    with open(os.path.join(world_dir, "level.dat"), "wb") as f:
        f.write(to_gzip("", root))
