"""BIP 360 P2MR construction e2e against living vectors under tests/vectors/p2mr/.

Both `p2mr_construction.json` and `p2mr_pqc_construction.json` get the same full
expected-value checks (leaf hashes, merkle root, scriptPubKey, address, control
blocks / leafVersion).
"""

from __future__ import annotations

import json
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent))

from p2mr.p2mr import (  # noqa: E402
    BIP360_tests,
    default_fixtures_dir,
    run_single_test,
)


class TestP2mrConstruction(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.fixtures = Path(default_fixtures_dir())
        cls.construction = cls.fixtures / "p2mr_construction.json"
        cls.pqc = cls.fixtures / "p2mr_pqc_construction.json"
        assert cls.construction.is_file(), f"missing {cls.construction}"
        assert cls.pqc.is_file(), f"missing {cls.pqc}"

    def _run_fixture(self, path: Path) -> None:
        with path.open(encoding="utf-8") as f:
            data = json.load(f)
        self.assertEqual(data.get("version"), 1)
        vectors = data["test_vectors"]
        self.assertGreater(len(vectors), 0)
        failed = []
        for i, v in enumerate(vectors):
            if not run_single_test(v, i + 1):
                failed.append(v.get("id", str(i)))
        self.assertEqual(failed, [], f"failed vectors in {path.name}: {failed}")

    def test_p2mr_construction_json(self) -> None:
        """Classic construction vectors (full expected-value checks)."""
        self._run_fixture(self.construction)

    def test_p2mr_pqc_construction_json(self) -> None:
        """PQC construction vectors (same full checks as classic)."""
        self._run_fixture(self.pqc)

    def test_bip360_tests_helper(self) -> None:
        self.assertEqual(BIP360_tests(str(self.construction)), 0)
        self.assertEqual(BIP360_tests(str(self.pqc)), 0)


if __name__ == "__main__":
    unittest.main()
