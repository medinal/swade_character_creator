#!/bin/bash

# Change to the directory where this script is located
cd "$(dirname "$0")"

# Base directory for all dumps
DUMPS_BASE="table_dumps"

# Create base directory if it doesn't exist
mkdir -p "$DUMPS_BASE"

# Determine the new version directory name
# Look for existing version directories (v001, v002, etc.)
latest_version=$(ls -d "$DUMPS_BASE"/v* 2>/dev/null | sort -V | tail -n 1 | grep -o 'v[0-9]*')

if [ -z "$latest_version" ]; then
    # No existing versions, start with v001
    new_version="v001"
else
    # Extract number, increment, and format
    version_num=$(echo "$latest_version" | sed 's/v//')
    # Force base-10 interpretation by removing leading zeros
    new_num=$((10#$version_num + 1))
    new_version=$(printf "v%03d" $new_num)
fi

# Also create a timestamp-based directory name for reference
timestamp=$(date +"%Y%m%d_%H%M%S")

# Create the new version directory
version_dir="$DUMPS_BASE/$new_version"
mkdir -p "$version_dir"

echo "=== Generating Table Dumps ==="
echo "Version: $new_version"
echo "Timestamp: $timestamp"
echo "Directory: $version_dir"
echo ""

# Get list of all tables
tables=$(sqlite3 ./swade.db "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name;")

# Dump each table
table_count=0
for table in $tables; do
    echo "Dumping table: $table"
    sqlite3 ./swade.db ".dump $table" > "$version_dir/${table}.sql"
    table_count=$((table_count + 1))
done

# Create a metadata file
cat > "$version_dir/dump_info.txt" << METADATA
SWADE Database Table Dump
=========================
Version: $new_version
Timestamp: $timestamp
Date: $(date)
Tables Dumped: $table_count
Database: ./swade.db

Tables:
$(echo "$tables" | sed 's/^/  - /')
METADATA

# Create a symlink to the latest version
rm -f "$DUMPS_BASE/latest"
ln -s "$new_version" "$DUMPS_BASE/latest"

echo ""
echo "=== Dump Complete ==="
echo "Tables dumped: $table_count"
echo "Location: $version_dir"
echo "Symlink 'latest' updated to point to $new_version"
echo ""
echo "Previous versions:"
ls -d "$DUMPS_BASE"/v* 2>/dev/null | sort -V | sed 's/^/  /'
