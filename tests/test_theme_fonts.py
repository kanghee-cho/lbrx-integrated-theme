import contextlib
import io
from pathlib import Path
import runpy
import tempfile
import unittest
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[1]
main = runpy.run_path(str(ROOT / "scripts" / "theme-fonts.py"))["main"]
tool = main.__globals__


class FontTests(unittest.TestCase):
    def setUp(self):
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        self.root = Path(temp.name)
        self.state, self.fonts = self.root / "state", self.root / "fonts"
        self.assets = self.root / "assets"
        self.assets.mkdir()
        self.data = b"font fixture"
        self.file = dict(filename="Exact-Regular.ttf", sha256=tool["sha"](self.data),
                         windows_name="Exact Regular (TrueType)")
        self.asset = self.assets / self.file["filename"]
        self.asset.write_bytes(self.data)
        self.font = dict(id="exact", family="Exact ligature", version="1", files=[self.file])
        self.inputs = ({"exact ligature": "Exact ligature"}, {"exact ligature": self.font})

    def invoke(self, args, families, verify=True):
        out, err = io.StringIO(), io.StringIO()
        with (
            patch.dict(tool, dict(
                ASSETS=self.assets,
                load_inputs=lambda _: self.inputs,
                locations=lambda: (self.state, self.fonts),
                installed_families=lambda: families,
                verify_family=lambda _: verify,
                refresh=lambda _: None,
                install_lock=lambda _: contextlib.nullcontext(),
            )),
            patch.object(tool["sys"], "platform", "test"),
            contextlib.redirect_stdout(out),
            contextlib.redirect_stderr(err),
        ):
            result = main(args)
        return result, out.getvalue(), err.getvalue()

    def test_exact_family_and_dry_run_do_not_create_directories(self):
        result, output, _ = self.invoke(["install", "--dry-run"], {"exact"})
        self.assertEqual(result, 0)
        self.assertIn("Exact ligature: missing", output)
        self.assertIn(str(self.fonts / "Exact-Regular.ttf"), output)
        self.assertIn(str(self.asset), output)
        self.assertEqual(list(self.root.iterdir()), [self.assets])
        self.assertIn("Exact ligature: external", self.invoke([], {"exact ligature"})[1])

    def test_install_tracks_ownership_and_refuses_drift(self):
        self.assertEqual(self.invoke(["install"], set())[0], 0)
        path = self.fonts / "Exact-Regular.ttf"
        self.assertEqual(path.read_bytes(), self.data)
        state = self.state / "state.json"
        previous = state.stat().st_mtime_ns
        self.assertIn("Exact ligature: managed", self.invoke(["install"], {"exact ligature"})[1])
        self.assertEqual(state.stat().st_mtime_ns, previous)
        path.write_bytes(b"user edits")
        result, output, _ = self.invoke(["install"], {"exact ligature"})
        self.assertEqual(result, 1)
        self.assertIn("Exact ligature: drifted", output)
        self.assertEqual(path.read_bytes(), b"user edits")

    def test_explicit_legacy_family_alias_is_detected_without_matching_other_styles(self):
        self.font["family_aliases"] = ["Exact Ligature Regular"]
        installed = {"exact ligature regular"}
        self.assertIn("Exact ligature: external", self.invoke(["install"], installed)[1])
        self.assertFalse(self.fonts.exists())
        self.assertIn("Exact ligature: missing", self.invoke([], {"exact ligature bold"})[1])
        with patch.dict(tool, installed_families=lambda: installed):
            self.assertTrue(tool["verify_family"](self.font))

    def test_failed_registration_rolls_back_without_touching_other_fonts(self):
        self.fonts.mkdir()
        external = self.fonts / "External.ttf"
        external.write_bytes(b"external")
        result, _, error = self.invoke(["install"], set(), verify=False)
        self.assertEqual(result, 1)
        self.assertIn("OS did not recognize", error)
        self.assertEqual(list(self.fonts.iterdir()), [external])
        self.assertEqual(external.read_bytes(), b"external")
        self.assertFalse((self.state / "state.json").exists())
        self.assertFalse((self.state / "pending.json").exists())

    def test_pending_install_blocks_changes(self):
        self.state.mkdir()
        pending = self.state / "pending.json"
        pending.write_text("interrupted", encoding="utf-8")
        result, _, error = self.invoke(["install"], set())
        self.assertEqual(result, 1)
        self.assertIn(str(pending), error)
        self.assertFalse(self.fonts.exists())

    def test_missing_and_modified_local_assets_block_before_writes(self):
        self.asset.unlink()
        self.assertEqual(self.invoke(["install"], {"exact ligature"})[0], 0)
        for args in (["install"], ["install", "--dry-run"]):
            result, _, error = self.invoke(args, set())
            self.assertEqual(result, 1)
            self.assertIn(str(self.asset), error)
        self.asset.write_bytes(b"modified")
        result, _, error = self.invoke(["install"], set())
        self.assertEqual(result, 1)
        self.assertIn("font checksum", error)
        self.assertFalse(self.state.exists())
        self.assertFalse(self.fonts.exists())


if __name__ == "__main__":
    unittest.main()
