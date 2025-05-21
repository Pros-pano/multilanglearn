#!/bin/bash

echo "🔍 Listing *user-installed* software on Ubuntu"
echo "============================================="

# Get list of manually installed packages excluding the original base system packages
echo -e "\n📦 APT Packages (manually installed by user):"
comm -23 \
  <(apt-mark showmanual | sort) \
  <(gzip -dc /var/log/installer/initial-status.gz | \
    grep "^Package: " | cut -d' ' -f2 | sort)

# Snap packages
if command -v snap &>/dev/null; then
  echo -e "\n📦 Snap Packages:"
  snap list | awk 'NR==1 || $5 != "canonical"'  # skip built-in base/core snaps
else
  echo -e "\n⚠️ Snap is not installed."
fi

# Flatpak packages
if command -v flatpak &>/dev/null; then
  echo -e "\n📦 Flatpak Packages:"
  flatpak list --app
else
  echo -e "\n⚠️ Flatpak is not installed."
fi

# Optional: pip packages (exclude system packages)
if command -v pip3 &>/dev/null; then
  echo -e "\n🐍 Python pip packages (user only):"
  pip3 list --user
fi

# Optional: npm global packages
if command -v npm &>/dev/null; then
  echo -e "\n📦 NPM global packages:"
  npm list -g --depth=0 | grep -v 'npm@'
fi

# Optional: cargo packages
if command -v cargo &>/dev/null; then
  echo -e "\n🦀 Rust (cargo) packages:"
  cargo install --list
fi

echo -e "\n✅ Done: Only user-installed packages listed."
