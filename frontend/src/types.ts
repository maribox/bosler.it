export const API_URL: string = "https://bosler.it/api";
export enum Visibility {
    Public,
    Private
}
export interface File {
    name: string;
    type: string;
    sizeInB: number;
    visibility: Visibility;
    createdAt: Date;
    uploadedAt: Date;
    downloadUrl: string;
    description: string;
}


