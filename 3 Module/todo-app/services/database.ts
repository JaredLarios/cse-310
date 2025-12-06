import * as SQLite from "expo-sqlite";

let db: SQLite.SQLiteDatabase | null = null;

export async function getDatabase() {
    if (!db) {
        db = await SQLite.openDatabaseAsync("database.db");
    }
    return db;
}

export async function initDatabase() {
    const database = await getDatabase();

    await database.execAsync(`
        CREATE TABLE IF NOT EXISTS items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0
        );
    `);

    console.log("📦 Database initialized");
}