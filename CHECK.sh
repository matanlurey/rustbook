
#!/bin/bash
set -e

echo "This script checks that Rust appers to be installed correctly..."
echo ""

echo "rustc ⏳"
rustc --version | sed 's/^/ ╠══ /'
echo "rustc ✅"

echo ""
echo "Rust appears to be installed!"
exit 0
