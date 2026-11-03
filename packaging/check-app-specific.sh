#!/usr/bin/env bash
# App-specific package assertions retained alongside shared packaging lint.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

python3 - <<'PYEOF'
import yaml
manifests = []
for name in ("com.iris.irisvector.yml", "com.iris.irisvector.bundle.yml"):
    with open("packaging/linux/flatpak/" + name) as source:
        manifest = yaml.safe_load(source)
    assert manifest["id"] == "com.iris.irisvector" and manifest["command"] == "vectorcraft", name
    manifests.append(manifest)
for key in ("runtime", "runtime-version", "sdk", "finish-args"):
    assert manifests[0][key] == manifests[1][key], f"Flatpak manifests disagree on {key}"
print("IrisVector package identity ok")
PYEOF
