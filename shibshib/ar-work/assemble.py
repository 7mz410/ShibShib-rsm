"""Build crates/ui-egui/src/i18n/ar.tsv from the translated batches (tr_NN next to part_NN).

    python3 shibshib/ar-work/assemble.py
"""
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
OUT = ROOT / 'crates/ui-egui/src/i18n/ar.tsv'
HEADER = """# Arabic (ar) catalog, ShibShib addition. Same format as zh-hant.tsv:  context <TAB> source <TAB> translation
#
# Written from the meaning of the English labels in Modern Standard Arabic, using the terms Arabic
# designers know for vector illustration (لوحة الرسم، الطبقات، التعبئة، الحدود، القلم، نقطة الربط).
# Plurals have six forms: zero|one|two|few|many|other (see `plural_arabic` in i18n/mod.rs).
# Rows are stored in logical order; i18n/bidi.rs puts them in visual order when the catalog loads.
#
# Coverage: being completed in batches (shibshib/ar-work). Strings without a row show in English.
"""


def lines(p: Path) -> list:
    out = p.read_text(encoding='utf-8').split('\n')
    return out[:-1] if out and out[-1] == '' else out


rows, seen = [], set()
for part in sorted(HERE.glob('part_*')):
    tr = HERE / part.name.replace('part_', 'tr_')
    if not tr.exists():
        continue
    keys, values = lines(part), lines(tr)
    assert len(keys) == len(values), (part.name, len(keys), len(values))
    for k, v in zip(keys, values):
        if k in seen:
            continue
        seen.add(k)
        rows.append(f'{k}\t{v}')
OUT.write_text(HEADER + '\n'.join(rows) + '\n', encoding='utf-8')
print(f'{len(rows)} rows -> {OUT.relative_to(ROOT)}')
