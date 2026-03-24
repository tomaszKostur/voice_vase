#!/bin/bash
DATABASE="voice_base.db"

export DATABASE_URL=$DATABASE

if [ -f $DATABASE ]; then
    rm $DATABASE
    echo "Deleted existing database"
fi

echo "Initializing SQLite database: $DATABASE"

cat ./voicebase_db/sqlite_dev_init.sql | sqlite3 "$DATABASE"

if [ $? -eq 0 ]; then
    echo "✅ Database created and initialized successfully!"
else
    echo "❌ Error occurred during initialization."
fi