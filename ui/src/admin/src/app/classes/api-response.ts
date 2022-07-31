// export class ApiResponse {
//     constructor(
//         status: String,
//         message: String,
//         data: JSON | null
//     ) {}
// }

export interface ApiResponse {
    status: string;
    message: string;
    data: JSON | null
}
