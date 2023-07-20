export interface Activity {
    name: string,
    body?: string
    tags: Array<Tag>
}

export type Activities = Map<number, Activity>

export interface Column {
    name: string,
    activities: Map<number, Activity>
}

export type Columns = Map<number, Column>

export interface Editable {
    id: number;
    field: ActiveField
}

export enum ActiveField {
    ActivityName = "activity name",
    ActivityBody = "activity body",
    ColumnName = "column name"
}


export interface Tag {
    name: string;
    categoryId?: number
}