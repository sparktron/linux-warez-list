#!/bin/bash
# Comprehensive software inventory for your Ubuntu system
# Run this on your actual machine: bash gather-software-inventory.sh > software-inventory.json

{
  echo "{"

  # APT packages
  echo '  "apt_packages": ['
  dpkg -l | grep '^ii' | awk '{print $2}' | while read pkg; do
    echo "    \"$pkg\","
  done | sed '$ s/,$//'
  echo "  ],"

  # Snap packages
  echo '  "snap_packages": ['
  snap list 2>/dev/null | tail -n +2 | awk '{print $1}' | while read pkg; do
    echo "    \"$pkg\","
  done | sed '$ s/,$//'
  echo "  ],"

  # Flatpak packages
  echo '  "flatpak_packages": ['
  flatpak list --app 2>/dev/null | tail -n +1 | awk '{print $1}' | while read pkg; do
    [ -n "$pkg" ] && echo "    \"$pkg\","
  done | sed '$ s/,$//'
  echo "  ],"

  # Python packages (global)
  echo '  "python_packages": ['
  pip list 2>/dev/null | tail -n +3 | awk '{print $1}' | while read pkg; do
    echo "    \"$pkg\","
  done | sed '$ s/,$//'
  echo "  ],"

  # Node packages (global)
  echo '  "npm_global_packages": ['
  npm list -g --depth=0 2>/dev/null | tail -n +2 | grep -o '^[^ ]*' | while read pkg; do
    [ -n "$pkg" ] && echo "    \"$pkg\","
  done | sed '$ s/,$//'
  echo "  ],"

  # Rust tools
  echo '  "rust_tools": ['
  ls ~/.cargo/bin 2>/dev/null | while read tool; do
    echo "    \"$tool\","
  done | sed '$ s/,$//'
  echo "  ],"

  # Fonts
  echo '  "fonts": ['
  find /usr/share/fonts -name "*.ttf" -o -name "*.otf" 2>/dev/null | xargs basename -a | sort -u | while read font; do
    echo "    \"$font\","
  done | sed '$ s/,$//'
  echo "  ],"

  # VSCode/Cursor extensions (if Cursor is installed)
  echo '  "editor_extensions": ['
  if [ -d ~/.config/Cursor/User/extensions ] || [ -d ~/.vscode/extensions ]; then
    find ~/.config/Cursor/User/extensions ~/.vscode/extensions -maxdepth 1 -type d 2>/dev/null | xargs -I {} basename {} | grep -v "^extensions$" | while read ext; do
      echo "    \"$ext\","
    done | sed '$ s/,$//'
  fi
  echo "  ],"

  # Desktop applications (check common locations)
  echo '  "desktop_applications": ['
  find /usr/share/applications ~/.local/share/applications -name "*.desktop" 2>/dev/null | xargs grep -h "^Name=" | cut -d= -f2 | sort -u | while read app; do
    echo "    \"$app\","
  done | sed '$ s/,$//'
  echo "  ]"

  echo "}"
} > software-inventory.json

echo "✅ Inventory saved to software-inventory.json"
