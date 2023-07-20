import type { Activities, Activity, Column, Columns } from "./interfaces/main";

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


const newActivities: Map<number, Activity> = new Map([
    [1, { name: "homework", tags: [{name: "Homework", id: 1}] }],
    [2, { name: "extra homework", body: "presentation", tags: [{name: "Homework", id: 1}]}]
]);

const inProgressActivities: Map<number, Activity> = new Map([
    [3, {name: "household chores", tags: [{name: "Chore", id: 2}]}]
]);

const doneActivities: Map<number, Activity> = new Map([
    [4, { name: "gym trenning", body: "gazylion push ups", tags: [{ name: "Gym", id: 2 }] }],
    [5, {name: "PE trenning", body: "gazylion push ups", tags: [{name: "Gym", id: 2}, {name: "Homework", id: 1}]}]
]);

export const mockColumns: Map<number, Column> = new Map([
    [1, { name: "New", activities: newActivities }],
    [2, { name: "In progress", activities: inProgressActivities }],
    [3, { name: "Done", activities: doneActivities }]
]);