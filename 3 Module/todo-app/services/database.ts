import * as SQLite from "expo-sqlite";

let db: SQLite.SQLiteDatabase | null = null;

// Returns a single shared database instance.
// Ensures the database is only opened once.
export async function getDatabase() {
    if (!db) {
        db = await SQLite.openDatabaseAsync("database.db");
    }
    return db;
}

// Initializes the database by creating tables if they don't exist.
// This sets up the "items" table used for the to-do list.
export async function initDatabase() {
    const database = await getDatabase();
    
    // Create the items table with id, name, and completed fields
    await database.execAsync(`
        CREATE TABLE IF NOT EXISTS items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0
        );
    `);

    console.log("📦 Database initialized");
}