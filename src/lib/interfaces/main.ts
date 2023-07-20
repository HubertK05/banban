export interface Activity {
    name: string,
    body?: string
}

export type Activities = Map<number, Activity>

export interface Column {
    name: string,
    activities: Map<number, Activity>
}

export type Columns = Map<number, Column>