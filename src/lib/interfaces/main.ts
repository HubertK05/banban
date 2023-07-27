export interface BackendColumn {
    name?: string,
    columnOrdinal?: number,
    activities: Record<number,
        {
            title: string,
            body?: string,
            categoryTags: Record<number, {
                categoryName: string,
                categoryOrdinal: number,
                tagId: number
                tagName: string,
                tagOrdinal: number
            }>,
            otherTags: Record<number, string>,
            activityOrdinal: number
        }>
}

export interface Activity {
    name: string,
    body?: string
    tags: Array<number>
    ordinal: number
}

export type Activities = Map<number, Activity>

export interface Column {
    name: string,
    activities: Map<number, Activity>,
    ord: number
}

export type Columns = Map<number, Column>

export type NonColumnActivities = Map<number, Activity>

export interface Tag {
    name: string;
    color?: string
    ord: number
}

export type Tags = Map<number, Tag>


export interface Category {
    name: string,
    tags: Array<number>,
    ord: number
}

export type Categories = Map<number, Category>

export type NonCategoryTags = Map<number, Tag>

export interface Editable {
    id: number;
    field: ActiveField
}

export enum ActiveField {
    ActivityName = "activity name",
    ActivityBody = "activity body",
    ColumnName = "column name"
}

export enum DrawerTab {
    Activity = "activity",
    Settings = "settings",
    OtherActivities = "otherActivities"
}
