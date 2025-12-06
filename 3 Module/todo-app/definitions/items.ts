export interface TaskI {
    id: number,
    text: string,
    completed: boolean 
}

export interface TaskDB {
    id: number,
    name: string,
    completed: boolean
}

export interface ItemI {
    task: TaskI,
    deleteTask: (id: number) => void,
    toggleCompleted: (id: number) => void,
}