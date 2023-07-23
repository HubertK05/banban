import type { Activities, Activity, Categories, Category, Column, Columns, Tag } from "./interfaces/main";

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

export const mockTags: Map<number, Tag> = new Map([
    [1, { name: "🏝 Low", ord: 3 }],
    [2, { name: "🏕 Medium", ord: 1 }],
    [3, { name: "🏔 High", ord: 2 }],
    [4, { name: "🌋 Urgent", ord: 4 }],
    [5, { name: "🦔 Tiny", ord: 1 }],
    [6, { name: "🐇 Small", ord: 2 }],
    [7, { name: "🐂 Medium", ord: 3 }],
    [8, { name: "🦑 Large", ord: 4 }],
    [9, { name: "🐋 X-Large", ord: 5 }]

])
const sizeCategory: Category = { name: "Size", tags: [1, 2, 3, 4], ord: 3 }
const priorityCategory: Category = { name: "Priority", tags: [5, 6, 7, 8, 9], ord: 2 }
const emptyCategory: Category = { name: "Complexity", tags: [], ord: 1 }

export const baseCategories: Categories = new Map([
    [1, sizeCategory],
    [2, priorityCategory],
    [3, emptyCategory]
])

const newActivities: Map<number, Activity> = new Map([
    [1, { name: "homework", tags: [1, 7], ord: 1 }],
    [2, { name: "extra homework", body: "presentation", tags: [3, 7], ord: 2 }]
]);

const inProgressActivities: Map<number, Activity> = new Map([
    [3, { name: "household chores", tags: [1, 9], ord: 1 }]
]);

const doneActivities: Map<number, Activity> = new Map([
    [4, { name: "gym workout", body: "gazylion push ups", tags: [9], ord: 1 }],
    [5, { name: "PE workout", body: "gazylion push ups", tags: [4, 7], ord: 2 }]
]);

export const mockColumns: Map<number, Column> = new Map([
    [1, { name: "New", activities: newActivities, ord: 1 }],
    [2, { name: "In progress", activities: inProgressActivities, ord: 2 }],
    [3, { name: "Done", activities: doneActivities, ord: 3 }]
]);