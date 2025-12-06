import React, { useEffect, useState } from 'react';
import { View, TextInput, Button, StyleSheet } from 'react-native';
import TodoItem from './TodoItem';

import { getItems, addTodoItem, deleteItem, updateItemStatus } from '../models/item';

import { TaskI } from '../definitions/items';

export default function TodoList() {
    const [tasks, setTasks] = useState<TaskI[]>([]);
    const [text, setText] = useState('');

    // Load items when the component mounts
    useEffect(() => {
        loadItems();
    }, []);

    // Fetch items from the database and update state
    async function loadItems() {
        const items = await getItems();
        setTasks(items);
    }

    // Add a new task to SQLite
    async function addTask() {
        // Ignore empty or whitespace-only input
        if (!text.trim()) return;
        await addTodoItem({
            text, completed: false,
        });
        await loadItems();

        setText('');
    }

    // Delete a task by its ID
    async function deleteTask(id: number) {
        await deleteItem({id});
        await loadItems();
    }

    // Toggle the completed status for a task
    async function toggleCompleted(id: number) {
        await updateItemStatus({id});
        await loadItems();
    }

    return (
        <View>
            {/* Input section */}
            <View style={styles.inputs}>
                <TextInput
                    value={text}
                    onChangeText={setText}
                    placeholder="New Task"
                    style={styles.textInput}
                />

                <View style={{ width: '15%' }}>
                    <Button title="Add" onPress={addTask} />
                </View>
            </View>

            {/* Task list */}
            <View style={styles.container}>
                {tasks.map(task => (
                    <TodoItem
                        key={task.id}
                        task={task}
                        deleteTask={deleteTask}
                        toggleCompleted={toggleCompleted}
                    />
                ))}
            </View>
        </View>
    );
}

const styles = StyleSheet.create({
    container: {
        flexDirection: 'column',
        marginTop: 8,
    },
    inputs: {
        flexDirection: 'row',
        gap: 5,
        margin: 5,
        alignItems: 'center',
        width: '100%',
    },
    textInput: {
        width: '80%',
        borderWidth: 1,
        padding: 8,
        borderRadius: 10,
    }
});
