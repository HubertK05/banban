import type { Activities, Activity, Categories, Column, Columns, Tag } from "./interfaces/main";

export const stringToColour = (str: string) => {
    let hash = 0;
    str.split('').forEach(char => {
        hash = char.charCodeAt(0) + ((hash << 5) - hash)
    })
    let colour = '#'
    for (let i = 0; i < 3; i++) {
        const value = (hash >> (i * 8)) & 0xff
        colour += value.toString(16).padStart(2, '0')
    }
    return colour
}

const sizeTags: Array<Tag> = [{ name: "Small", id: 1, categoryId: 1 }, { name: "Medium", id: 2, categoryId: 1 }, { name: "Big", id: 3, categoryId: 1 }]
const priorityTags: Array<Tag> = [{ name: "Low", id: 4, categoryId: 2 }, { name: "Medium", id: 5, categoryId: 2 }, { name: "High", id: 6, categoryId: 2 }, { name: "Urgent", id: 7, categoryId: 2 }]

export const baseCategories: Categories = new Map([
    [1, { name: "Size", tags: sizeTags }],
    [2, { name: "Priority", tags: priorityTags }]
])

const newActivities: Map<number, Activity> = new Map([
    [1, { name: "homework", tags: [{ name: "Homework", id: 8 }] }],
    [2, { name: "extra homework", body: "presentation", tags: [{ name: "Homework", id: 8 }] }]
]);

const inProgressActivities: Map<number, Activity> = new Map([
    [3, { name: "household chores", tags: [{ name: "Chore", id: 9 }] }]
]);

const doneActivities: Map<number, Activity> = new Map([
    [4, { name: "gym workout", body: "gazylion push ups", tags: [{ name: "Gym", id: 10 }] }],
    [5, { name: "PE workout", body: "gazylion push ups", tags: [{ name: "Gym", id: 10 }, { name: "Homework", id: 8 }] }]
]);

export const mockColumns: Map<number, Column> = new Map([
    [1, { name: "New", activities: newActivities }],
    [2, { name: "In progress", activities: inProgressActivities }],
    [3, { name: "Done", activities: doneActivities }]
]);