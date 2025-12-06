import { TaskDB, TaskI } from "../definitions/items";
import { getDatabase } from '../services/database';

// Add a new task to the SQLite database
export async function addTodoItem(item: Omit<TaskI, 'id'>): Promise<boolean> {
    // Insert a new row into the items table
    await (await getDatabase()).runAsync(`
        INSERT INTO items (name, completed) VALUES (?, ?);
        `, [item.text, item.completed])

    return true;
}

// Get all tasks from the database
export async function getItems(): Promise<TaskI[]> {
    // Fetch all items and order them (incomplete first)
    const allItems = await (await getDatabase()).getAllAsync(`
        SELECT * FROM items ORDER BY completed ASC, id ASC;;
    `) as TaskDB[];

    // Convert DB rows into TaskI objects used by the app
    return allItems.map((item) => ({
        id: item.id,
        text: item.name,
        completed: item.completed
    }));
}


// Toggle the completed status of a task
export async function updateItemStatus(task: Pick<TaskI, 'id'>): Promise<boolean> {
    // Get the task's current state
    const item = await (await getDatabase()).getFirstAsync(
        `SELECT * FROM items WHERE id = ?;`, [task.id]) as TaskDB;

    // Update the completed value (flip true/false)
    await (await getDatabase()).runAsync(`UPDATE items SET completed = ? WHERE id = ?;`, [!item.completed, task.id]);
    return true
}

// Delete a task by ID
export async function deleteItem(task: Pick<TaskI, 'id'>): Promise<boolean> {
    // Remove the task from the database
    await (await getDatabase()).runAsync(`DELETE FROM items WHERE id = ?`, [task.id]);
    return true
}
