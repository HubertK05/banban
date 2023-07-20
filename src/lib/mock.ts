import type { Activities, Activity, Column, Columns } from "./interfaces/main";

const newActivities: Map<number, Activity> = new Map([
    [1, { name: "homework" }],
    [2, { name: "extra homework", body: "presentation"}]
]);

const inProgressActivities: Map<number, Activity> = new Map([
    [3, {name: "household chores"}]
]);

const doneActivities: Map<number, Activity> = new Map([
    [4, {name: "gym trenning", body: "gazylion push ups"}]
]);

export const mockColumns: Map<number, Column> = new Map([
    [1, { name: "New", activities: newActivities }],
    [2, { name: "In progress", activities: inProgressActivities }],
    [3, { name: "Done", activities: doneActivities }]
]);