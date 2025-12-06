import { TaskDB, TaskI } from "../definitions/items";
import { getDatabase } from '../services/database';

export async function addTodoItem(item: Omit<TaskI, 'id'>): Promise<boolean> {
    await (await getDatabase()).runAsync(`
        INSERT INTO items (name, completed) VALUES (?, ?);
        `, [item.text, item.completed])

    return true;
}

export async function getItems(): Promise<TaskI[]> {
    const allItems = await (await getDatabase()).getAllAsync(`
        SELECT * FROM items ORDER BY completed ASC, id ASC;;
    `) as TaskDB[];

    return allItems.map((item) => ({
        id: item.id,
        text: item.name,
        completed: item.completed
    }));
}

export async function updateItemStatus(task: Pick<TaskI, 'id'>): Promise<boolean> {
    const item = await (await getDatabase()).getFirstAsync(
        `SELECT * FROM items WHERE id = ?;`, [task.id]) as TaskDB;

    await (await getDatabase()).runAsync(`UPDATE items SET completed = ? WHERE id = ?;`, [!item.completed, task.id]);
    return true
}

export async function deleteItem(task: Pick<TaskI, 'id'>): Promise<boolean> {
    await (await getDatabase()).runAsync(`DELETE FROM items WHERE id = ?`, [task.id]);
    return true
}
