#!/data/data/com.termux/files/usr/bin/python
"""zipalign replacement: rewrite an APK so that
- lib/*.so entries are STORED and their data starts at a 16 KiB boundary
- resources.arsc is STORED and 4-byte aligned
- everything else keeps its original method; stored entries align to 4

Usage: apkalign.py <in.apk> <out.apk>
"""
import struct
import sys
import zlib


def main(src: str, dst: str) -> None:
    data = open(src, "rb").read()
    eocd = data.rfind(b"PK\x05\x06")
    if eocd < 0:
        raise ValueError("EOCD not found")
    count = struct.unpack_from("<H", data, eocd + 10)[0]
    cd_off = struct.unpack_from("<I", data, eocd + 16)[0]

    entries = []
    pos = cd_off
    for _ in range(count):
        sig, = struct.unpack_from("<I", data, pos)
        assert sig == 0x02014b50, "bad central directory entry"
        (ver_made, ver_need, flags, method, mtime, mdate, crc,
         csize, usize, nlen, elen, clen, disk, iattr, eattr, lho) = \
            struct.unpack_from("<HHHHHHIIIHHHHHII", data, pos + 4)
        name = data[pos + 46:pos + 46 + nlen].decode("utf-8")
        cd_extra = data[pos + 46 + nlen:pos + 46 + nlen + elen]
        comment = data[pos + 46 + nlen + elen:pos + 46 + nlen + elen + clen]
        # local header: signature at lho, fields start at lho + 4
        sig_l, = struct.unpack_from("<I", data, lho)
        assert sig_l == 0x04034b50, f"bad local header for {name}"
        (lver, lflags, lmethod, lmtime, lmdate, lcrc, lcsize, lusize, lnlen, lelen) = \
            struct.unpack_from("<HHHHHIIIHH", data, lho + 4)
        name_bytes = data[lho + 30:lho + 30 + lnlen]
        payload = data[lho + 30 + lnlen + lelen:lho + 30 + lnlen + lelen + lcsize]
        if lflags & 0x8:
            raise ValueError("data descriptor entries not supported")
        if method == 8:
            raw = zlib.decompress(payload, -15)
        else:
            raw = payload
        entries.append({
            "name": name, "ver_made": ver_made, "ver_need": ver_need,
            "flags": lflags, "mtime": lmtime, "mdate": lmdate,
            "raw": raw, "comment": comment, "eattr": eattr,
        })
        pos += 46 + nlen + elen + clen

    with open(dst, "wb") as out:
        new_entries = []
        for e in entries:
            name_b = e["name"].encode("utf-8")
            store = e["name"].startswith("lib/") and e["name"].endswith(".so")
            if store or e["name"] == "resources.arsc":
                method = 0
                align = 16384 if store else 4
            else:
                method = 8
                align = 0

            if method == 0:
                payload = e["raw"]
                crc = zlib.crc32(payload) & 0xFFFFFFFF
                csize = usize = len(payload)
            else:
                co = zlib.compressobj(9, zlib.DEFLATED, -15)
                payload = co.compress(e["raw"]) + co.flush()
                crc = zlib.crc32(e["raw"]) & 0xFFFFFFFF
                csize = len(payload)
                usize = len(e["raw"])

            extra = b""
            offset = out.tell()
            if align:
                pad = (align - (offset + 30 + len(name_b) + len(extra)) % align) % align
                extra = b"\x00" * pad

            new_offset = out.tell()
            out.write(struct.pack(
                "<IHHHHHIIIHH", 0x04034b50, 20, e["flags"] & ~0x8, method,
                e["mtime"], e["mdate"], crc, csize, usize,
                len(name_b), len(extra)))
            out.write(name_b)
            out.write(extra)
            out.write(payload)
            new_entries.append({
                "name": e["name"], "ver_made": e["ver_made"], "ver_need": e["ver_need"],
                "flags": e["flags"] & ~0x8, "method": method, "mtime": e["mtime"],
                "mdate": e["mdate"], "crc": crc, "csize": csize, "usize": usize,
                "extra": b"", "comment": e["comment"], "offset": new_offset,
                "eattr": e["eattr"],
            })

        cd_start = out.tell()
        for ne in new_entries:
            name_b = ne["name"].encode("utf-8")
            out.write(struct.pack(
                "<IHHHHHHIIIHHHHHII", 0x02014b50, ne["ver_made"], ne["ver_need"],
                ne["flags"], ne["method"], ne["mtime"], ne["mdate"], ne["crc"],
                ne["csize"], ne["usize"], len(name_b), len(ne["extra"]),
                len(ne["comment"]), 0, 0, ne["eattr"], ne["offset"]))
            out.write(name_b)
            out.write(ne["extra"])
            out.write(ne["comment"])
        cd_end = out.tell()
        out.write(struct.pack(
            "<IHHHHIIH", 0x06054b50, 0, 0, len(new_entries), len(new_entries),
            cd_end - cd_start, cd_start, 0))
    print(f"aligned: {dst}")


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
