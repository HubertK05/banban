export interface Activity {
    name: string;
    body?: string;
    tags: Array<number>;
    ordinal: number;
}

export interface Column {
    name: string;
    activities: number[];
    ord: number;
}

export interface Tag {
    name: string;
    color: string;
    ord: number;
}

export interface Category {
    name: string;
    tags: Array<number>;
    ord: number;
}

export interface Editable {
    id: number;
    field: ActiveField;
}

export enum ActiveField {
    ActivityName = "activity name",
    ActivityBody = "activity body",
    ColumnName = "column name",
}

export enum DrawerTab {
    Activity = "activity",
    Settings = "settings",
    OtherActivities = "otherActivities",
}

export interface AppState {
    isDebug: boolean;
    previousDrawerTab: DrawerTab | null;
    currentEditable: Editable | null;
    selectedActivity: number | null;
    columnDragDisabled: boolean;
    hoverColumnId: null | number;
}
