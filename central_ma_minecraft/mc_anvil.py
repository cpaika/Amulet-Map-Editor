"""Write modern (1.18+) Minecraft Java Anvil region files from scratch.

A section's block grid is a (16,16,16) numpy array of palette indices in YZX
order (index = (y*16 + z)*16 + x). Uniform sections (single block) omit the
packed data array as required by the format.
"""
import os
import struct
import zlib
import math
import numpy as np

from mc_nbt import (Compound, List, Long, Int, Byte, String, LongArray,
                    IntArray, Float, TAG_COMPOUND, TAG_LONG, to_zlib)

SECTOR = 4096


def _bits_for(n, minbits):
    b = max(minbits, (n - 1).bit_length() if n > 1 else 1)
    return b


def pack_longs(indices, bits):
    """Non-spanning 1.16+ packing -> list[int] signed 64-bit longs."""
    epl = 64 // bits
    n = indices.shape[0]
    nlongs = (n + epl - 1) // epl
    pad = nlongs * epl - n
    if pad:
        indices = np.concatenate([indices, np.zeros(pad, dtype=np.uint64)])
    rows = indices.astype(np.uint64).reshape(nlongs, epl)
    out = np.zeros(nlongs, dtype=np.uint64)
    for k in range(epl):
        out |= rows[:, k] << np.uint64(bits * k)
    return out.view(np.int64).tolist()


def _palette_entry(nm):
    """nm is either 'minecraft:x' or ('minecraft:x', {'prop':'val'})."""
    if isinstance(nm, tuple):
        name, props = nm
        c = Compound({"Name": String(name)})
        if props:
            c["Properties"] = Compound({k: String(v) for k, v in props.items()})
        return c
    return Compound({"Name": String(nm)})


def make_block_states(section_idx, palette_names):
    """section_idx: (4096,) uint array of palette indices (YZX).
    palette_names: list of "minecraft:xxx" strings or (name, props) tuples.
    Returns a Compound for block_states."""
    pal_list = List(TAG_COMPOUND, [_palette_entry(nm) for nm in palette_names])
    comp = Compound({"palette": pal_list})
    if len(palette_names) > 1:
        bits = _bits_for(len(palette_names), 4)
        comp["data"] = LongArray(pack_longs(section_idx, bits))
    return comp


def make_biomes(biome_idx, palette_names):
    from mc_nbt import TAG_STRING
    pal_list = List(TAG_STRING, [String(nm) for nm in palette_names])
    comp = Compound({"palette": pal_list})
    if len(palette_names) > 1:
        bits = _bits_for(len(palette_names), 1)
        comp["data"] = LongArray(pack_longs(biome_idx, bits))
    return comp


def build_chunk(cx, cz, sections, data_version, biome="minecraft:plains",
                heightmap_ws=None):
    """sections: dict y_section_index -> (idx_array(4096,), palette_names list)
    or (None, [single_name]) for uniform.
    heightmap_ws: optional (256,) ints for MOTION_BLOCKING/WORLD_SURFACE."""
    sec_tags = []
    ys = sorted(sections.keys())
    ymin = min(ys) if ys else 0
    # ensure a contiguous-ish range; we only emit present sections plus they
    # must include any with blocks. Minecraft tolerates missing sections.
    biome_pal = [biome]
    for y in ys:
        idx, pal = sections[y]
        if idx is None:
            bs = make_block_states(None, pal)
        else:
            bs = make_block_states(idx, pal)
        sec = Compound({
            "Y": Byte(y),
            "block_states": bs,
            "biomes": make_biomes(None, biome_pal),
        })
        sec_tags.append(sec)

    root_data = Compound({
        "DataVersion": Int(data_version),
        "xPos": Int(cx),
        "zPos": Int(cz),
        "yPos": Int(-4),
        "Status": String("minecraft:full"),
        "LastUpdate": Long(0),
        "InhabitedTime": Long(0),
        "sections": List(TAG_COMPOUND, sec_tags),
        "block_entities": List(TAG_COMPOUND, []),
        "block_ticks": List(TAG_COMPOUND, []),
        "fluid_ticks": List(TAG_COMPOUND, []),
        "PostProcessing": List(9, []),  # list of lists
        "structures": Compound({"References": Compound({}), "starts": Compound({})}),
    })
    if heightmap_ws is not None:
        # 9 bits per height, 256 entries, packed non-spanning
        hm = pack_longs(np.asarray(heightmap_ws, dtype=np.uint64), 9)
        root_data["Heightmaps"] = Compound({
            "MOTION_BLOCKING": LongArray(hm),
            "WORLD_SURFACE": LongArray(hm),
        })
    # Root chunk tag is unnamed compound containing the data directly (1.18+)
    return root_data


class RegionWriter:
    """Collects chunk NBT and writes one r.x.z.mca file."""
    def __init__(self):
        self.chunks = {}  # (localx, localz) -> bytes (uncompressed nbt root)

    def add(self, local_cx, local_cz, chunk_root):
        from mc_nbt import write_named_root
        raw = write_named_root("", chunk_root)
        self.chunks[(local_cx, local_cz)] = zlib.compress(raw, 6)

    def write(self, path):
        header_loc = bytearray(SECTOR)
        header_time = bytearray(SECTOR)
        body = bytearray()
        cur_sector = 2  # after two header sectors
        for (lx, lz), comp in self.chunks.items():
            length = len(comp) + 1  # +1 for compression type byte
            chunk_bytes = struct.pack(">I", length) + b"\x02" + comp
            pad = (-len(chunk_bytes)) % SECTOR
            chunk_bytes += b"\x00" * pad
            nsec = len(chunk_bytes) // SECTOR
            i = (lx & 31) + (lz & 31) * 32
            struct.pack_into(">I", header_loc, i * 4,
                             (cur_sector << 8) | (nsec & 0xFF))
            struct.pack_into(">I", header_time, i * 4, 0)
            body += chunk_bytes
            cur_sector += nsec
        with open(path, "wb") as f:
            f.write(header_loc)
            f.write(header_time)
            f.write(body)
