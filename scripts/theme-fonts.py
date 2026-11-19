#!/usr/bin/env python3
"""Install missing LBRX font families for the current user."""

import argparse
from contextlib import contextmanager
import ctypes
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import time
import tomllib


ROOT = Path(__file__).resolve().parents[1]
ASSETS = ROOT / "fonts" / "assets"
GENERIC = {"sans-serif", "serif", "monospace", "system-ui", "cursive", "fantasy",
           "ui-serif", "ui-sans-serif", "ui-monospace", "ui-rounded",
           "math", "emoji", "fangsong"}
FONT_KEY = r"Software\Microsoft\Windows NT\CurrentVersion\Fonts"
MAX_BYTES = 100 * 1024 * 1024


def key(name):
    return name.strip().casefold()


def read_toml(path):
    with path.open("rb") as stream:
        return tomllib.load(stream)


def load_inputs(theme_path):
    typography = read_toml(theme_path)["typography"]
    required = {}
    for role in ("prose", "mono"):
        families = typography[role]["families"]
        if not isinstance(families, list) or not families:
            raise ValueError(f"{role}: expected a nonempty families array")
        for family in families:
            if not isinstance(family, str) or not key(family):
                raise ValueError(f"{role}: invalid font family")
            if key(family) not in GENERIC:
                required.setdefault(key(family), family.strip())
    catalog = read_toml(ROOT / "fonts" / "catalog.toml")
    if catalog["schema_version"] != 1:
        raise ValueError("Unsupported font catalog schema")
    return required, {key(font["family"]): dict(font, id=id_) for id_, font in catalog["fonts"].items()}


def xdg(variable, fallback):
    path = Path(os.environ.get(variable, ""))
    return path if path.is_absolute() else fallback


def locations():
    home = Path.home()
    if sys.platform == "win32":
        local = Path(os.environ["LOCALAPPDATA"])
        state, fonts = local, local / "Microsoft" / "Windows" / "Fonts"
    elif sys.platform == "darwin":
        state, fonts = home / "Library" / "Application Support", home / "Library" / "Fonts"
    elif sys.platform.startswith("linux"):
        state = xdg("XDG_STATE_HOME", home / ".local" / "state")
        fonts = xdg("XDG_DATA_HOME", home / ".local" / "share") / "fonts" / "lbrx-theme"
    else:
        raise ValueError(f"Unsupported platform: {sys.platform}")
    return state / "lbrx-theme" / "fonts", fonts


def windows_families():
    from ctypes import wintypes as w

    class LogFont(ctypes.Structure):
        _fields_ = [(name, w.LONG) for name in ("height", "width", "escapement", "orientation", "weight")]
        _fields_ += [(name, w.BYTE) for name in (
            "italic", "underline", "strikeout", "charset", "out_precision",
            "clip_precision", "quality", "pitch_and_family",
        )]
        _fields_ += [("face", w.WCHAR * 32)]

    callback = ctypes.WINFUNCTYPE(ctypes.c_int, ctypes.POINTER(LogFont), ctypes.c_void_p,
                                  w.DWORD, w.LPARAM)
    user, gdi = ctypes.windll.user32, ctypes.windll.gdi32
    user.GetDC.argtypes, user.GetDC.restype = [w.HWND], w.HDC
    user.ReleaseDC.argtypes = [w.HWND, w.HDC]
    gdi.EnumFontFamiliesExW.argtypes = [w.HDC, ctypes.POINTER(LogFont), callback, w.LPARAM, w.DWORD]
    families = set()

    @callback
    def collect(font, _metric, _kind, _data):
        families.add(key(font.contents.face))
        return 1

    dc = user.GetDC(None)
    if not dc:
        raise OSError("Could not obtain Windows font enumeration context")
    try:
        query = LogFont()
        query.charset = 1  # DEFAULT_CHARSET: enumerate all families.
        if not gdi.EnumFontFamiliesExW(dc, ctypes.byref(query), collect, 0, 0) or not families:
            raise OSError("Windows font enumeration failed")
        return families
    finally:
        user.ReleaseDC(None, dc)


def mac_families():
    core = ctypes.CDLL("/System/Library/Frameworks/CoreText.framework/CoreText")
    cf = ctypes.CDLL("/System/Library/Frameworks/CoreFoundation.framework/CoreFoundation")
    ptr, index = ctypes.c_void_p, ctypes.c_long
    copy = core.CTFontManagerCopyAvailableFontFamilyNames
    copy.argtypes, copy.restype = [], ptr
    cf.CFArrayGetCount.argtypes, cf.CFArrayGetCount.restype = [ptr], index
    cf.CFArrayGetValueAtIndex.argtypes, cf.CFArrayGetValueAtIndex.restype = [ptr, index], ptr
    cf.CFStringGetCString.argtypes = [ptr, ctypes.POINTER(ctypes.c_char), index, ctypes.c_uint32]
    cf.CFStringGetCString.restype = ctypes.c_bool
    cf.CFRelease.argtypes, cf.CFRelease.restype = [ptr], None
    array = copy()
    if not array:
        raise OSError("Core Text font enumeration failed")
    try:
        families = set()
        buffer = ctypes.create_string_buffer(4096)
        for position in range(cf.CFArrayGetCount(array)):
            name = cf.CFArrayGetValueAtIndex(array, position)
            if not cf.CFStringGetCString(name, buffer, len(buffer), 0x08000100):  # UTF-8
                raise OSError("Core Text returned an unreadable font family name")
            families.add(key(buffer.value.decode("utf-8")))
        return families
    finally:
        cf.CFRelease(array)


def run(*args):
    return subprocess.run(args, check=True, capture_output=True, encoding="utf-8", timeout=60).stdout


def installed_families():
    if sys.platform == "win32":
        return windows_families()
    if sys.platform == "darwin":
        return mac_families()
    output = run("fc-list", "-f", r"%{family}\n")
    return {key(family) for line in output.splitlines() for family in line.split(",") if key(family)}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def is_link(path):
    try:
        info = path.lstat()
    except FileNotFoundError:
        return False
    return path.is_symlink() or bool(getattr(info, "st_file_attributes", 0) & 0x400)


def read_state(path):
    if not os.path.lexists(path):
        return {}
    if is_link(path) or not path.is_file():
        raise ValueError(f"Not a regular state file: {path}")
    state = json.loads(path.read_text(encoding="utf-8"))
    if state["schema_version"] != 1 or not isinstance(state["fonts"], dict):
        raise ValueError(f"Invalid state file: {path}")
    return state["fonts"]


def atomic_json(path, value):
    fd, temp = tempfile.mkstemp(dir=path.parent, prefix=".fonts-", suffix=".tmp")
    try:
        with os.fdopen(fd, "w", encoding="utf-8") as file:
            json.dump(value, file, ensure_ascii=False, indent=2)
            file.write("\n")
            file.flush()
            os.fsync(file.fileno())
        os.replace(temp, path)
    finally:
        Path(temp).unlink(missing_ok=True)


@contextmanager
def install_lock(path):
    if is_link(path):
        raise ValueError(f"Refusing a linked lock file: {path}")
    with path.open("ab") as lock:
        try:
            if sys.platform == "win32":
                import msvcrt
                msvcrt.locking(lock.fileno(), msvcrt.LK_NBLCK, 1)
            else:
                import fcntl
                fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except OSError as error:
            raise OSError(f"Could not acquire font installation lock: {path}: {error}") from error
        yield


def target(font_dir, file):
    filename = file["filename"]
    if (Path(filename).name != filename or "/" in filename or "\\" in filename
            or ":" in filename or Path(filename).suffix.lower() not in {".ttf", ".otf", ".ttc"}):
        raise ValueError(f"Invalid catalog font filename: {filename!r}")
    return font_dir / filename


def intact(record, font_dir):
    files = record["files"]
    return bool(files) and all(
        (path := Path(file["path"])).parent == font_dir
        and not is_link(path) and path.is_file()
        and sha(path.read_bytes()) == file["sha256"]
        for file in files
    )


def family_available(family, font, installed):
    aliases = font.get("family_aliases", ()) if font else ()
    return key(family) in installed or any(key(alias) in installed for alias in aliases)


def statuses(required, catalog, state, font_dir, families):
    for id_, family in required.items():
        font = catalog.get(id_)
        if families is None:
            status = "unknown"
        elif id_ in state:
            status = ("managed" if family_available(family, font, families)
                      and intact(state[id_], font_dir) else "drifted")
        elif family_available(family, font, families):
            status = "external"
        else:
            status = "missing" if font else "unknown"
        yield family, status, font


def read_assets(font):
    if is_link(ASSETS):
        raise ValueError(f"Refusing a linked font assets directory: {ASSETS}")
    payload = []
    for file in font["files"]:
        path = target(ASSETS, file)
        if is_link(path) or not path.is_file():
            raise ValueError(f"{font['family']}: expected a regular local font file: {path}")
        with path.open("rb") as stream:
            data = stream.read(MAX_BYTES + 1)
        if len(data) > MAX_BYTES or sha(data) != file["sha256"]:
            raise ValueError(f"{font['family']}: font checksum or size mismatch: {path}")
        payload.append((file, data))
    return payload


def create_font(path, data):
    fd, temp = tempfile.mkstemp(dir=path.parent, prefix=".lbrx-font-", suffix=".tmp")
    try:
        with os.fdopen(fd, "wb") as stream:
            stream.write(data)
            stream.flush()
            os.fsync(stream.fileno())
        os.link(temp, path)  # Atomic no-overwrite publication, including on Windows.
    finally:
        Path(temp).unlink(missing_ok=True)


def windows_resource(path, *, remove=False):
    from ctypes import wintypes as w

    gdi = ctypes.windll.gdi32
    api = gdi.RemoveFontResourceExW if remove else gdi.AddFontResourceExW
    api.argtypes = [w.LPCWSTR, w.DWORD, ctypes.c_void_p]
    api.restype = w.BOOL if remove else ctypes.c_int
    if not api(str(path), 0, None):
        raise OSError(f"Windows could not {'unregister' if remove else 'register'} {path}")


def registry_value(name):
    import winreg

    try:
        with winreg.OpenKey(winreg.HKEY_CURRENT_USER, FONT_KEY) as registry:
            return winreg.QueryValueEx(registry, name)
    except FileNotFoundError:
        return None


def windows_registry(name, path, *, remove=False):
    import winreg

    value = registry_value(name)
    if not remove and value is not None:
        raise FileExistsError(f"Windows font registration already exists: {name}")
    if remove and value != (str(path), winreg.REG_SZ):
        raise ValueError(f"Windows font registration changed: {name}")
    with winreg.CreateKeyEx(winreg.HKEY_CURRENT_USER, FONT_KEY, 0, winreg.KEY_SET_VALUE) as registry:
        if remove:
            winreg.DeleteValue(registry, name)
        else:
            winreg.SetValueEx(registry, name, 0, winreg.REG_SZ, str(path))


def refresh(font_dir):
    if sys.platform == "win32":
        from ctypes import wintypes as w

        notify = ctypes.windll.user32.SendMessageTimeoutW
        notify.argtypes = [w.HWND, w.UINT, w.WPARAM, w.LPARAM, w.UINT, w.UINT, ctypes.c_void_p]
        notify.restype = w.LPARAM
        if not notify(0xFFFF, 0x001D, 0, 0, 0x0002, 200, None):
            print("Warning: font change notification timed out; reopen your apps.", file=sys.stderr)
    elif sys.platform.startswith("linux"):
        run("fc-cache", "-f", str(font_dir))


def verify_family(font):
    if sys.platform == "darwin":
        # A fresh process avoids Core Text's in-process cache while fontd notices the new files.
        for attempt in range(5):
            if attempt:
                time.sleep(0.5)
            installed = json.loads(run(sys.executable, str(Path(__file__).resolve()), "--probe"))
            if family_available(font["family"], font, installed):
                return True
        return False
    return family_available(font["family"], font, installed_families())


def install_font(font, payload, font_dir, state_dir, state):
    win = sys.platform == "win32"
    id_ = key(font["family"])
    files = []
    for file, _ in payload:
        path = target(font_dir, file)
        if os.path.lexists(path) or (win and registry_value(file["windows_name"]) is not None):
            raise FileExistsError(f"{font['family']}: existing file or registration for {path}")
        files.append(dict(path=str(path), sha256=file["sha256"], windows_name=file["windows_name"]))
    record = dict(id=font["id"], version=font["version"], files=files)
    journal = state_dir / "pending.json"
    atomic_json(journal, dict(schema_version=1, family=id_, record=record))
    created, registered, loaded = [], [], []
    try:
        for (file, data), entry in zip(payload, files, strict=True):
            path = Path(entry["path"])
            create_font(path, data)
            created.append((path, entry["sha256"]))
            if win:
                name = file["windows_name"]
                windows_registry(name, path)
                registered.append((name, path))
                windows_resource(path)
                loaded.append(path)
        refresh(font_dir)
        if not verify_family(font):
            raise OSError(f"{font['family']}: OS did not recognize the installed family")
    except (OSError, ValueError, subprocess.SubprocessError, KeyboardInterrupt) as error:
        try:
            for path in reversed(loaded):
                windows_resource(path, remove=True)
            for name, path in reversed(registered):
                windows_registry(name, path, remove=True)
            for path, digest in reversed(created):
                if is_link(path) or not path.is_file() or sha(path.read_bytes()) != digest:
                    raise ValueError(f"Font file changed during rollback: {path}")
                path.unlink()
            if created:
                refresh(font_dir)
            journal.unlink()
        except (OSError, ValueError, subprocess.SubprocessError) as rollback:
            raise OSError(f"{error}; rollback failed: {rollback}; inspect {journal}") from rollback
        raise
    # Keep the journal if publishing state fails; the committed state may already be visible.
    atomic_json(state_dir / "state.json", dict(schema_version=1, fonts=state | {id_: record}))
    state[id_] = record
    journal.unlink()
    print(f"{font['family']}: installed ({font['version']})")


def inspect(required, catalog, state, font_dir):
    try:
        families = installed_families()
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        print(f"Font detection failed: {error}", file=sys.stderr)
        families = None
    return list(statuses(required, catalog, state, font_dir, families))


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", nargs="?", choices=("status", "install"), default="status")
    parser.add_argument("--dry-run", action="store_true", help="show the install plan without changes")
    parser.add_argument("--theme", type=Path, default=ROOT / "themes" / "lbrx.toml")
    parser.add_argument("--probe", action="store_true", help=argparse.SUPPRESS)
    args = parser.parse_args(argv)
    if args.dry_run and args.command != "install":
        parser.error("--dry-run requires install")
    try:
        if args.probe:
            print(json.dumps(sorted(installed_families())))
            return 0
        required, catalog = load_inputs(args.theme)
        state_dir, font_dir = locations()
        if is_link(state_dir) or is_link(font_dir):
            raise ValueError("Refusing linked font or state directory")
        state_file, journal = state_dir / "state.json", state_dir / "pending.json"
        rows = inspect(required, catalog, read_state(state_file), font_dir)
        for family, status, font in rows:
            print(f"{family}: {status}")
            if args.dry_run and status == "missing":
                print(f"  {font['version']}")
                for file in font["files"]:
                    print(f"  {target(ASSETS, file)} -> {target(font_dir, file)}")
        pending = os.path.lexists(journal)
        if pending:
            print(f"Install interrupted or in progress; inspect {journal}", file=sys.stderr)
        blocked = any(status in ("unknown", "drifted") for _, status, _ in rows)
        if args.command == "status":
            return int(pending or blocked)
        if blocked or pending:
            return 1
        prepared = {font["id"]: read_assets(font) for _, status, font in rows if status == "missing"}
        if args.dry_run or not prepared:
            return 0
        if sys.platform.startswith("linux") and shutil.which("fc-cache") is None:
            raise OSError("fc-cache is unavailable; install your distribution's Fontconfig tools")
        state_dir.mkdir(parents=True, exist_ok=True)
        with install_lock(state_dir / "install.lock"):
            if os.path.lexists(journal):
                raise ValueError(f"Interrupted install; inspect {journal}")
            state = read_state(state_file)
            rows = inspect(required, catalog, state, font_dir)
            if any(status in ("unknown", "drifted") for _, status, _ in rows):
                raise ValueError("Font state changed; run status before retrying")
            plan = [(font, prepared[font["id"]] if font["id"] in prepared else read_assets(font))
                    for _, status, font in rows if status == "missing"]
            for font, payload in plan:
                font_dir.mkdir(parents=True, exist_ok=True)
                install_font(font, payload, font_dir, state_dir, state)
        return 0
    except (OSError, ValueError, KeyError, TypeError, subprocess.SubprocessError) as error:
        print(f"Font tool: {error}", file=sys.stderr)
        return 1
    except KeyboardInterrupt:
        print("Font tool: interrupted", file=sys.stderr)
        return 130


if __name__ == "__main__":
    sys.exit(main())
