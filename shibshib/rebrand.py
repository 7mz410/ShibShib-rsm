#!/usr/bin/env python3
"""Apply the ShibShib rsm branding on top of upstream VectorCraft.

Only user-visible text is rebranded; crate names, file formats and other internals keep their
upstream names so `git merge upstream/main` stays easy. The script is idempotent: after merging
upstream, resolve any conflicts in favour of upstream and run it again.

    python3 shibshib/rebrand.py
"""
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
NAME = 'ShibShib rsm'
REPO = 'https://github.com/7mz410/ShibShib-rsm'
# Persisted names that saved preferences and files refer to; renaming them would break those.
KEEP = ('VectorCraft Default',)

# Exact replacements, applied before the general rename.
SPECIFIC = {
    'crates/engine/src/cmd/help.rs': [
        ('pub const WEBSITE_URL: &str = "https://getartcraft.com";', f'pub const WEBSITE_URL: &str = "{REPO}";'),
        ('format!("{WEBSITE_URL}/apps/{APP_ID}")', 'WEBSITE_URL.to_string()'),
        ('format!("https://github.com/storytold/{APP_ID}")', 'WEBSITE_URL.to_string()'),
        ('"Join Our Discord"', '"Upstream Community on Discord"'),
        ('the ArtCraft community Discord', 'the upstream ArtCraft community Discord'),
        ('"ArtCraft Website"', '"ShibShib Project"'),
        ('["url"], "https://getartcraft.com/apps/vectorcraft");', f'["url"], "{REPO}");'),
        ('["url"], "https://github.com/storytold/vectorcraft");', f'["url"], "{REPO}");'),
        ('["website"], "https://getartcraft.com");', f'["website"], "{REPO}");'),
    ],
    'crates/engine/src/cmd/fileio/mod.rs': [
        ('label: "VectorCraft",', f'label: "{NAME}",'),
        ('label: "VectorCraft Template",', f'label: "{NAME} Template",'),
    ],
    # No ShibShib Discord yet: drop the upstream community button and the ArtCraft website link.
    'crates/ui-egui/src/chrome.rs': [
        ('let discord = room >= with_search + 10.0 + gap + crate::community::discord_width(ui, false);',
         'let discord = false; // ShibShib: no community Discord button yet'),
    ],
    'crates/ui-egui/src/titlebar.rs': [
        ('assert_eq!((has("Discord"), has("Search commands and tools")), (width > 1000.0, width > 1000.0), "{width}: {texts:?}");',
         'assert_eq!((has("Discord"), has("Search commands and tools")), (false, width > 1000.0), "{width}: {texts:?}");'),
    ],
    'crates/ui-egui/src/community.rs': [
        ('''    discord_button(app, ui, true);
    ui.add_space(8.0);
''', ''),
        ('''    link(app, ui, "globe", tl!("ArtCraft website"), "help.website", &s("website"));
''', ''),
        ('''                "https://getartcraft.com",
                "https://getartcraft.com/apps/vectorcraft",
                "https://github.com/storytold/vectorcraft"''',
         f'''                "{REPO}",
                "{REPO}",
                "{REPO}"'''),
    ],
    'crates/ui-egui/src/dialogs/about.rs': [
        ('"Part of ArtCraft. MIT OR Apache-2.0.',
         '"Based on VectorCraft by the ArtCraft team. MIT OR Apache-2.0.'),
    ],
}
# Exact replacements inside string literals and catalogs, before the general rename.
LITERALS = [
    ('VectorCraft on getartcraft.com', f'{NAME} Website'),
    ('ArtCraft Website', 'ShibShib Project'),
    ('Join Our Discord', 'Upstream Community on Discord'),
]
# Catalog rows for strings the rebranded UI no longer shows.
DROPPED_ROWS = {'ArtCraft website'}
# The About credit line must keep naming VectorCraft.
CREDIT = 'Based on VectorCraft by the ArtCraft team'

STRING = re.compile(r'"(?:[^"\\\n]|\\.)*"')


def rename(text: str) -> str:
    for a, b in LITERALS:
        text = text.replace(a, b)
    if CREDIT in text or any(k in text for k in KEEP):
        return text
    return text.replace('VectorCraft', NAME)


def rebrand_rust(path: Path) -> bool:
    src = path.read_text(encoding='utf-8')
    out = src
    for a, b in SPECIFIC.get(path.relative_to(ROOT).as_posix(), []):
        out = out.replace(a, b)
    out = STRING.sub(lambda m: rename(m.group(0)), out)
    if out != src:
        path.write_text(out, encoding='utf-8')
    return out != src


def source_strings() -> set:
    """Every string literal in the rebranded sources, without quotes."""
    found = set()
    for p in RUST_FILES():
        found.update(m.group(0)[1:-1] for m in STRING.finditer(p.read_text(encoding='utf-8')))
    return found


def rebrand_catalog(path: Path, sources: set) -> bool:
    # Rows are `context<TAB>English<TAB>translation`. A row is renamed only when its renamed English
    # text is a string the rebranded sources use; rows for other crates' messages keep matching them.
    src = path.read_text(encoding='utf-8')
    rows = []
    for line in src.split('\n'):
        cells = line.split('\t')
        if len(cells) >= 3 and not line.startswith('#') and cells[1] in DROPPED_ROWS:
            continue
        if len(cells) >= 3 and not line.startswith('#'):
            renamed = rename(cells[1])
            if renamed != cells[1] and renamed in sources:
                line = '\t'.join([cells[0], renamed, *(rename(c) for c in cells[2:])])
        rows.append(line)
    out = '\n'.join(rows)
    if out != src:
        path.write_text(out, encoding='utf-8')
    return out != src


def rebrand_plist(path: Path) -> bool:
    src = path.read_text(encoding='utf-8')
    out = re.sub(r'(<key>CFBundle(?:Display)?Name</key>\s*<string>)VectorCraft(</string>)', rf'\g<1>{NAME}\2', src)
    out = out.replace('<string>ai.storyteller.vectorcraft</string>', '<string>com.shibshib.rsm</string>')
    if out != src:
        path.write_text(out, encoding='utf-8')
    return out != src


def RUST_FILES():
    return sorted((ROOT / 'crates/ui-egui/src').rglob('*.rs')) + [ROOT / p for p in SPECIFIC if p.startswith('crates/engine')]


def main():
    changed = []
    for p in RUST_FILES():
        if rebrand_rust(p):
            changed.append(p)
    sources = source_strings()
    for p in sorted((ROOT / 'crates/ui-egui/src/i18n').glob('*.tsv')):
        if rebrand_catalog(p, sources):
            changed.append(p)
    web = ROOT / 'apps/vectorcraft-web/index.html'
    html = web.read_text(encoding='utf-8')
    branded = html.replace('<title>VectorCraft</title>', f'<title>{NAME}</title>').replace('Loading VectorCraft&hellip;', f'Loading {NAME}&hellip;')
    if branded != html:
        web.write_text(branded, encoding='utf-8')
        changed.append(web)
    if rebrand_plist(ROOT / 'packaging/macos/Info.plist.in'):
        changed.append(ROOT / 'packaging/macos/Info.plist.in')
    for p in changed:
        print('rebranded', p.relative_to(ROOT))


if __name__ == '__main__':
    main()
