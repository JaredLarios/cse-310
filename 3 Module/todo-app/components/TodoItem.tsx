import React from 'react';
import Checkbox from 'expo-checkbox';
import { View, Text, TouchableOpacity, StyleSheet } from 'react-native';
import { ItemI } from '../definitions/items'


export default function TodoItem({ task, deleteTask, toggleCompleted }: ItemI) {
    return (
        <View style={styles.todo_item}>
            <Checkbox
                value={task.completed === 1}
                onValueChange={() => toggleCompleted(task.id)}
            />
            <Text style={[styles.todo_item_text, task.completed && styles.completed]}>
                {task.text}
            </Text>
            <TouchableOpacity
                style={styles.delete_button}
                onPress={() => deleteTask(task.id)}
            >
                <Text style={{ color: '#fff' }}>Delete</Text>
            </TouchableOpacity>
        </View>
    );
}

const styles = StyleSheet.create({
    todo_item: {
        flexDirection: 'row',
        justifyContent: 'space-between',
        alignItems: 'center',
        padding: 10,
        marginBottom: 8,
        borderWidth: 1,
        borderColor: '#ddd',
        borderRadius: 4,
        width: '90%',
        marginHorizontal: 'auto'
    },

    todo_item_text: {
        marginRight: 8,
        color: '#555',
        fontSize: 16,
    },

    delete_button: {
        backgroundColor: '#ff6347',
        paddingHorizontal: 10,
        paddingVertical: 6,
        borderRadius: 4,
    },

    completed: {
        textDecorationStyle: 'dashed',
        color: '#888'
    }
});