"""Minimal Minecraft NBT writer (big-endian, Java edition).

Only the tag types needed to write modern (1.18+) Anvil chunks are implemented.
Tags are represented as small Python wrapper classes so that ambiguous Python
types (e.g. int -> byte/int/long) are encoded unambiguously.
"""
import struct
import gzip
import zlib

# Tag type ids
TAG_END = 0
TAG_BYTE = 1
TAG_SHORT = 2
TAG_INT = 3
TAG_LONG = 4
TAG_FLOAT = 5
TAG_DOUBLE = 6
TAG_BYTE_ARRAY = 7
TAG_STRING = 8
TAG_LIST = 9
TAG_COMPOUND = 10
TAG_INT_ARRAY = 11
TAG_LONG_ARRAY = 12


class Tag:
    tid = None
    def write_payload(self, buf):
        raise NotImplementedError


class Byte(Tag):
    tid = TAG_BYTE
    __slots__ = ("v",)
    def __init__(self, v): self.v = int(v)
    def write_payload(self, buf): buf += struct.pack(">b", self.v)


class Short(Tag):
    tid = TAG_SHORT
    __slots__ = ("v",)
    def __init__(self, v): self.v = int(v)
    def write_payload(self, buf): buf += struct.pack(">h", self.v)


class Int(Tag):
    tid = TAG_INT
    __slots__ = ("v",)
    def __init__(self, v): self.v = int(v)
    def write_payload(self, buf): buf += struct.pack(">i", self.v)


class Long(Tag):
    tid = TAG_LONG
    __slots__ = ("v",)
    def __init__(self, v): self.v = int(v)
    def write_payload(self, buf): buf += struct.pack(">q", self.v)


class Float(Tag):
    tid = TAG_FLOAT
    __slots__ = ("v",)
    def __init__(self, v): self.v = float(v)
    def write_payload(self, buf): buf += struct.pack(">f", self.v)


class Double(Tag):
    tid = TAG_DOUBLE
    __slots__ = ("v",)
    def __init__(self, v): self.v = float(v)
    def write_payload(self, buf): buf += struct.pack(">d", self.v)


class String(Tag):
    tid = TAG_STRING
    __slots__ = ("v",)
    def __init__(self, v): self.v = v
    def write_payload(self, buf):
        b = self.v.encode("utf-8")
        buf += struct.pack(">H", len(b)); buf += b


class ByteArray(Tag):
    tid = TAG_BYTE_ARRAY
    __slots__ = ("v",)
    def __init__(self, v): self.v = v  # bytes
    def write_payload(self, buf):
        buf += struct.pack(">i", len(self.v)); buf += bytes(self.v)


class IntArray(Tag):
    tid = TAG_INT_ARRAY
    __slots__ = ("v",)
    def __init__(self, v): self.v = v  # list[int]
    def write_payload(self, buf):
        buf += struct.pack(">i", len(self.v))
        buf += struct.pack(">%di" % len(self.v), *self.v)


class LongArray(Tag):
    tid = TAG_LONG_ARRAY
    __slots__ = ("v",)
    def __init__(self, v): self.v = v  # list[int] (signed 64)
    def write_payload(self, buf):
        buf += struct.pack(">i", len(self.v))
        if self.v:
            buf += struct.pack(">%dq" % len(self.v), *self.v)


class List(Tag):
    tid = TAG_LIST
    __slots__ = ("item_tid", "items")
    def __init__(self, item_tid, items):
        self.item_tid = item_tid; self.items = items
    def write_payload(self, buf):
        buf += struct.pack(">b", self.item_tid)
        buf += struct.pack(">i", len(self.items))
        for it in self.items:
            it.write_payload(buf)


class Compound(Tag):
    tid = TAG_COMPOUND
    __slots__ = ("d",)
    def __init__(self, d=None): self.d = d or {}
    def __setitem__(self, k, v): self.d[k] = v
    def write_payload(self, buf):
        for name, tag in self.d.items():
            buf += struct.pack(">b", tag.tid)
            nb = name.encode("utf-8")
            buf += struct.pack(">H", len(nb)); buf += nb
            tag.write_payload(buf)
        buf += b"\x00"  # TAG_END


def write_named_root(name, tag):
    """Serialize a root tag with a name (usually empty) -> bytes."""
    buf = bytearray()
    buf += struct.pack(">b", tag.tid)
    nb = name.encode("utf-8")
    buf += struct.pack(">H", len(nb)); buf += nb
    tag.write_payload(buf)
    return bytes(buf)


def to_gzip(name, tag):
    return gzip.compress(write_named_root(name, tag))


def to_zlib(name, tag):
    return zlib.compress(write_named_root(name, tag))
