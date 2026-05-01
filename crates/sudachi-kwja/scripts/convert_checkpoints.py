"""Convert KWJA Lightning .ckpt files to safetensors.

Run once per upstream KWJA release. Output goes to the directory passed via --out;
the production location is ~/.local/share/jisho/checkpoints/.

Production cache filenames (KWJA v2.4):
    /root/.cache/kwja/v2.4/char_deberta-v2-base-wwm.ckpt
    /root/.cache/kwja/v2.4/word_deberta-v2-base.ckpt

These live inside the running container at jisho-parse-1:/root/.cache/kwja/.
Copy them to a host-readable path (e.g. ~/.cache/kwja-export/) via
`docker cp` before running this script.

Usage:
    cd packages/rs/kwja/scripts
    uv run python convert_checkpoints.py \\
        --in ~/.cache/kwja-export/char_deberta-v2-base-wwm.ckpt \\
        --out ~/.local/share/jisho/checkpoints/char.safetensors
"""

from __future__ import annotations

import argparse
import hashlib
import sys
from pathlib import Path

import torch
from safetensors.torch import save_file


def convert(ckpt_path: Path, out_path: Path) -> str:
    """Load Lightning checkpoint, extract state_dict, save as safetensors. Returns sha256."""
    ckpt = torch.load(ckpt_path, map_location="cpu", weights_only=False)
    state = ckpt["state_dict"] if isinstance(ckpt, dict) and "state_dict" in ckpt else ckpt
    cleaned = {
        k.removeprefix("model."): v.contiguous()
        for k, v in state.items()
        if isinstance(v, torch.Tensor)
    }
    out_path.parent.mkdir(parents=True, exist_ok=True)
    save_file(cleaned, str(out_path))
    return hashlib.sha256(out_path.read_bytes()).hexdigest()


def main() -> int:
    p = argparse.ArgumentParser()
    p.add_argument("--in", dest="ckpt", type=Path, required=True)
    p.add_argument("--out", dest="out", type=Path, required=True)
    args = p.parse_args()

    sha = convert(args.ckpt.expanduser(), args.out.expanduser())
    print(f"Wrote {args.out} ({args.out.expanduser().stat().st_size:,} bytes)")
    print(f"sha256: {sha}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
