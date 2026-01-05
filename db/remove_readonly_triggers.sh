#!/bin/bash

# Change to the directory where this script is located
cd "$(dirname "$0")"

# Script to remove read-only triggers from an existing database
# Use this when you need to modify seed data in an existing database

DB_PATH="./swade.db"

echo "=== Removing Read-Only Triggers ==="
echo ""

if [ ! -f "$DB_PATH" ]; then
    echo "  ✗ Error: Database not found at $DB_PATH"
    exit 1
fi

echo "Listing current triggers..."
trigger_count=$(sqlite3 "$DB_PATH" "SELECT COUNT(*) FROM sqlite_master WHERE type='trigger' AND name LIKE 'prevent_%';")
echo "  Found $trigger_count read-only triggers"
echo ""

if [ "$trigger_count" -eq 0 ]; then
    echo "  ℹ No read-only triggers to remove"
    exit 0
fi

echo "Removing triggers..."

# Generate DROP TRIGGER statements and execute them
sqlite3 "$DB_PATH" "
SELECT 'DROP TRIGGER ' || name || ';' 
FROM sqlite_master 
WHERE type = 'trigger' AND name LIKE 'prevent_%';
" | sqlite3 "$DB_PATH"

if [ $? -eq 0 ]; then
    echo "  ✓ All read-only triggers removed"
    echo ""
    echo "  ⚠ Warning: Seed data tables are now writable!"
    echo "  ⚠ Be careful not to accidentally modify game rules data"
    echo ""
    echo "To reapply triggers:"
    echo "  1. Run: ./reset_and_load.sh (rebuilds database with triggers)"
    echo "  2. Or run: sqlite3 swade.db < create_readonly_triggers.sql (applies triggers to existing database)"
else
    echo "  ✗ Error removing triggers"
    exit 1
fi

echo ""
echo "=== Complete ==="
