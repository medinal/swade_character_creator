#!/bin/bash

# Change to the directory where this script is located
cd "$(dirname "$0")"

DB_PATH="./swade.db"
SCHEMA_FILE="./schema.sql"
DUMPS_BASE="./table_dumps"
TRIGGERS_FILE="./create_readonly_triggers.sql"

# Parse command line arguments
APPLY_TRIGGERS="yes"  # Default: apply triggers
VERSION="latest"      # Default: latest version

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --no-triggers)
            APPLY_TRIGGERS="no"
            shift
            ;;
        --triggers)
            APPLY_TRIGGERS="yes"
            shift
            ;;
        *)
            # Assume it's a version number
            VERSION="$1"
            shift
            ;;
    esac
done

echo "=== SWADE Database Reset and Load ==="
echo ""

# Step 1: Remove existing database
echo "Step 1: Removing existing database..."
if [ -f "$DB_PATH" ]; then
    rm -f "$DB_PATH"
    echo "  ✓ Existing database removed"
else
    echo "  ℹ No existing database found"
fi

# Step 2: Create fresh database with schema
echo ""
echo "Step 2: Creating database from schema..."
if [ ! -f "$SCHEMA_FILE" ]; then
    echo "  ✗ Error: $SCHEMA_FILE not found!"
    exit 1
fi
sqlite3 "$DB_PATH" < "$SCHEMA_FILE"
echo "  ✓ Schema loaded"

# Step 3: Determine which version to load
echo ""
echo "Step 3: Loading seed data..."

# Resolve version directory
if [ "$VERSION" = "latest" ]; then
    DUMPS_DIR="$DUMPS_BASE/latest"
    if [ ! -L "$DUMPS_DIR" ]; then
        echo "  ✗ Error: No 'latest' symlink found. Run generate_table_dumps.sh first."
        exit 1
    fi
    ACTUAL_VERSION=$(readlink "$DUMPS_DIR")
    echo "  Version: $ACTUAL_VERSION (latest)"
else
    DUMPS_DIR="$DUMPS_BASE/$VERSION"
    ACTUAL_VERSION="$VERSION"
    echo "  Version: $VERSION"
fi

if [ ! -d "$DUMPS_DIR" ]; then
    echo "  ✗ Error: Version directory not found: $DUMPS_DIR"
    echo ""
    echo "Available versions:"
    ls -d "$DUMPS_BASE"/v* 2>/dev/null | sed 's|.*/||' | sed 's/^/    /'
    exit 1
fi

# Show dump info if available
if [ -f "$DUMPS_DIR/dump_info.txt" ]; then
    echo ""
    grep -E "^(Version|Timestamp|Date):" "$DUMPS_DIR/dump_info.txt" | sed 's/^/  /'
fi

echo ""

# Define table order (respecting foreign key dependencies)
# Only include tables with seed data (exclude character tables)
seed_tables=(
    "attributes"
    "skills"
    "ranks"
    "modifiers"
    "requirements"
    "requirement_expressions"
    "ancestries"
    "ancestry_modifiers"
    "ancestry_requirements"
    "edges"
    "edge_modifiers"
    "edge_requirements"
    "hindrances"
    "hindrance_modifiers"
    "hindrance_requirements"
    "arcane_backgrounds"
    "arcane_background_requirements"
    "powers"
    "power_modifiers"
    "power_requirements"
    "ancestry_choices"
    "ancestry_choice_options"
    "gear_categories"
    "gear"
    "weapon_stats"
    "armor_stats"
    "shield_stats"
    "ammunition_stats"
    "poison_stats"
    "pack_contents"
    "gear_modifiers"
    "gear_requirements"
)

# Extract only INSERT statements from each dump file
for table in "${seed_tables[@]}"; do
    dump_file="$DUMPS_DIR/${table}.sql"
    
    if [ -f "$dump_file" ]; then
        echo "  Loading: $table"
        # Extract only INSERT and PRAGMA statements, skip CREATE TABLE and indexes
        grep -E "^(INSERT|PRAGMA)" "$dump_file" | sqlite3 "$DB_PATH"
        
        if [ $? -eq 0 ]; then
            echo "    ✓ Success"
        else
            echo "    ✗ Error loading $table"
            exit 1
        fi
    else
        echo "    ⚠ Warning: $dump_file not found, skipping"
    fi
done

echo ""
echo "Step 4: Managing read-only triggers..."

if [ "$APPLY_TRIGGERS" = "yes" ]; then
    if [ -f "$TRIGGERS_FILE" ]; then
        sqlite3 "$DB_PATH" < "$TRIGGERS_FILE"
        if [ $? -eq 0 ]; then
            echo "  ✓ Read-only triggers applied"
            echo "  ℹ Seed data tables are now protected from modification"
        else
            echo "  ✗ Error applying triggers"
            exit 1
        fi
    else
        echo "  ⚠ Warning: $TRIGGERS_FILE not found, skipping triggers"
    fi
else
    echo "  ℹ Skipping triggers (--no-triggers flag specified)"
    echo "  ⚠ Warning: Seed data tables are NOT protected"
fi

echo ""
echo "=== Database Reset Complete ==="
echo ""
echo "Summary:"
sqlite3 "$DB_PATH" "SELECT
    (SELECT COUNT(*) FROM attributes) as attributes,
    (SELECT COUNT(*) FROM skills) as skills,
    (SELECT COUNT(*) FROM edges) as edges,
    (SELECT COUNT(*) FROM hindrances) as hindrances,
    (SELECT COUNT(*) FROM powers) as powers,
    (SELECT COUNT(*) FROM ancestries) as ancestries,
    (SELECT COUNT(*) FROM gear) as gear;"

echo ""
if [ "$APPLY_TRIGGERS" = "yes" ]; then
    echo "Protection: Seed data tables are protected by triggers (read-only)"
    echo "            Character tables (character_*, characters) remain writable"
else
    echo "Protection: NONE - All tables are writable (--no-triggers was used)"
    echo "            Use with caution! Seed data can be accidentally modified"
fi
echo ""
echo "Usage: ./reset_and_load.sh [OPTIONS] [VERSION]"
echo ""
echo "Options:"
echo "  --triggers      Apply read-only triggers (default)"
echo "  --no-triggers   Skip applying triggers (for development/testing)"
echo ""
echo "Examples:"
echo "  ./reset_and_load.sh                    # Load latest with triggers"
echo "  ./reset_and_load.sh v001               # Load v001 with triggers"
echo "  ./reset_and_load.sh --no-triggers      # Load latest without triggers"
echo "  ./reset_and_load.sh --no-triggers v001 # Load v001 without triggers"
echo "  ./reset_and_load.sh v001 --no-triggers # Load v001 without triggers (order doesn't matter)"
