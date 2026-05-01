"""Generate per-head equivalence fixtures from KWJA-Python.

Run inside jisho-parse-1 container which has KWJA-Python + the inference
helpers already in place. This script imports directly from the service
sources (`/app/services/jisho-parse/python/`), instantiates a KwjaState,
then runs each canonical input through the model and dumps every emitted
field into per-head JSON files.

Usage:
    docker cp gen_fixtures.py jisho-parse-1:/tmp/gen_fixtures.py
    docker exec jisho-parse-1 python /tmp/gen_fixtures.py /tmp/fixtures
    docker cp jisho-parse-1:/tmp/fixtures /tmp/fixtures   # outside container

Outputs into <out_dir>:
    word_features.json        list of {input, expected: List[List[str]]}
    bp_features.json          list of {input, expected: List[Dict[str,str]]}
    dependency_types.json     list of {input, expected: List[str]}
    cohesion.json             list of {input, expected: List[List[{type,target}]]}
    discourse.json            list of {input, expected: List[{from_*, to_*, type}]}
    typo.json                 list of {input, expected: str}  (handcrafted pairs)

Notes on extraction:
  * `word_features` come from `morpheme.features` (rhoknp). KWJA writes
    multi-label feature strings into this dict; we list keys in stable
    sort order so test comparisons are order-independent.
  * `bp_features` come from `base_phrase.features` (rhoknp dict-style).
  * `dep_types` come from `base_phrase.dep_type`.
  * `cohesion` comes from `base_phrase.rel_tags` (PAS + bridging + coref).
  * `discourse` comes from `document.discourse_relations` (set if KWJA
    found cross-sentence discourse pairs; empty otherwise).
  * `typo` is hand-curated from KWJA's evaluation set since the raw model
    output is text→text — we don't need a Document tree there.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

# jisho-parse-py source layout in the container.
sys.path.insert(0, "/app/services/jisho-parse/python")

# Single-sentence corpus — every parser exercises one sentence.
SINGLE_SENTENCE_CORPUS = [
    "今日は晴れです。",
    "夜になって、雨も風も静まった。",
    "次の十字路を左に曲がって、まっすぐ行ってください。",
    "４月と１０月に歓迎会がある。",
    "お風呂のお湯を抜く。",
    "彼は毎朝コーヒーを飲みながら新聞を読む。",
    "明日の会議は午後三時から始まります。",
    "彼女が部屋に入ったとき、みんなが拍手した。",
    "雨が降りそうだから傘を持って行ったほうがいい。",
    "新幹線で京都まで二時間半かかります。",
]

# Multi-sentence corpus — needed for discourse extraction (cross-sentence).
DISCOURSE_CORPUS = [
    "雨が降った。だから試合は中止になった。",
    "彼は熱があった。それでも会議に出た。",
    "目的を明確にしよう。そうすれば成功する。",
    "彼女は早起きだ。一方、彼は夜型だ。",
    "風邪をひいた。なぜなら寒い場所にいたからだ。",
]

# Typo correction: hand-curated input/expected pairs. KWJA's typo model
# fixes character-level errors (duplications, kana confusions, wrong kanji).
TYPO_CORPUS: list[tuple[str, str]] = [
    ("今日は天気がいいいいですね", "今日は天気がいいですね"),
    ("わたしわ学生です", "わたしは学生です"),
    ("コンビニにいって買いものをした", "コンビニに行って買い物をした"),
]


# KWJA's word_feature_tagger emits exactly these 5 multi-labels.
# Source: kwja.utils.constants.WORD_FEATURES.
KWJA_WORD_FEATURES = {
    "基本句-主辞",
    "基本句-区切",
    "文節-区切",
    "用言表記先頭",
    "用言表記末尾",
}


def _morpheme_features(m) -> list[str]:
    """Extract sorted KWJA word_feature_tagger labels from a rhoknp Morpheme.

    rhoknp.Morpheme.features mixes multiple sources (KWJA tagger output,
    rhoknp spelling-variant ALT-* annotations, JumanDIC fields). We filter
    to only the 5 word_features KWJA's tagger actually predicts so the
    fixture matches what our Rust port will emit.
    """
    if not getattr(m, "features", None):
        return []
    return sorted(
        k for k, v in m.features.items()
        if k in KWJA_WORD_FEATURES and (v is True or v == "true")
    )


def _bp_features(bp) -> dict[str, str]:
    """Extract dict-style features from a rhoknp BasePhrase."""
    out: dict[str, str] = {}
    for k, v in (getattr(bp, "features", {}) or {}).items():
        if v is True:
            out[k] = "true"
        elif isinstance(v, (str, int, float)):
            out[k] = str(v)
        # Skip lists/dicts/None for stability.
    return out


def _bp_relations(bp) -> list[dict]:
    """Extract PAS / bridging / coreference relations from a rhoknp BasePhrase."""
    out = []
    for rel in getattr(bp, "rel_tags", []) or []:
        out.append({
            "type": rel.type,
            "target": rel.target,
            "sid": rel.sid if hasattr(rel, "sid") else "",
            "id": str(rel.target_id) if hasattr(rel, "target_id") else "",
        })
    return out


def _document_discourse(doc) -> list[dict]:
    """Extract cross-sentence discourse predictions from a rhoknp Document."""
    out = []
    # rhoknp models discourse as inter-clause relations on each clause.
    sents = list(doc.sentences)
    sent_index_of_bp = {}
    for si, sent in enumerate(sents):
        for bp in sent.base_phrases:
            sent_index_of_bp[id(bp)] = si
    for sent_idx, sent in enumerate(sents):
        for bp in sent.base_phrases:
            for rel in getattr(bp, "discourse_relations", []) or []:
                target = rel.modifier if hasattr(rel, "modifier") else rel.target
                target_sent_idx = sent_index_of_bp.get(id(target), -1)
                target_bp_idx = target.index if hasattr(target, "index") else -1
                rel_type = rel.label if hasattr(rel, "label") else str(rel)
                if target_sent_idx < 0:
                    continue
                out.append({
                    "from_sentence": sent_idx,
                    "to_sentence": target_sent_idx,
                    "from_base_phrase": bp.index,
                    "to_base_phrase": target_bp_idx,
                    "type": rel_type,
                })
    return out


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("out_dir", type=Path)
    args = ap.parse_args()
    args.out_dir.mkdir(parents=True, exist_ok=True)

    # Lazy imports so the script can be syntax-checked outside the container.
    from inference import KwjaState, load_model, parse_chunk  # type: ignore

    print("loading model...", flush=True)
    state: KwjaState = load_model()
    print(f"ready: device={state.device}", flush=True)

    word_features: list[dict] = []
    bp_features: list[dict] = []
    dep_types: list[dict] = []
    cohesion: list[dict] = []

    for text in SINGLE_SENTENCE_CORPUS:
        docs = parse_chunk(state, [text])
        if not docs:
            continue
        doc = docs[0]
        sents = list(doc.sentences)
        if not sents:
            continue
        sent = sents[0]
        word_features.append({
            "input": text,
            "expected": [_morpheme_features(m) for m in sent.morphemes],
        })
        bp_features.append({
            "input": text,
            "expected": [_bp_features(bp) for bp in sent.base_phrases],
        })
        dep_types.append({
            "input": text,
            "expected": [
                bp.dep_type.value if hasattr(bp.dep_type, "value") else str(bp.dep_type)
                for bp in sent.base_phrases
            ],
        })
        cohesion.append({
            "input": text,
            "expected": [_bp_relations(bp) for bp in sent.base_phrases],
        })
        print(f"  done: {text[:30]}", flush=True)

    discourse: list[dict] = []
    for text in DISCOURSE_CORPUS:
        docs = parse_chunk(state, [text])
        if not docs:
            continue
        doc = docs[0]
        discourse.append({
            "input": text,
            "expected": _document_discourse(doc),
        })
        print(f"  discourse: {text[:30]}", flush=True)

    typo = [{"input": pre, "expected": post} for pre, post in TYPO_CORPUS]

    def _write(name: str, data) -> None:
        p = args.out_dir / name
        p.write_text(json.dumps(data, ensure_ascii=False, indent=2))
        print(f"  wrote {p} ({len(data)} cases)", flush=True)

    _write("word_features.json", word_features)
    _write("bp_features.json", bp_features)
    _write("dependency_types.json", dep_types)
    _write("cohesion.json", cohesion)
    _write("discourse.json", discourse)
    _write("typo.json", typo)
    print(f"done: 6 fixture files in {args.out_dir}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
