// export class ApiResponse {
//     constructor(
//         status: String,
//         message: String,
//         data: JSON | null
//     ) {}
// }

export interface ApiResponse {
    status: boolean;
    message: string;
    data: JSON | null
}
