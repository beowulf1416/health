// export class ApiResponse {
//     constructor(
//         status: String,
//         message: String,
//         data: JSON | null
//     ) {}
// }

export interface ApiResponse {
    success: boolean;
    message: string;
    data: JSON | null
}

export function newApiResponse(
    success: boolean,
    message: string,
    data: JSON | null
) : ApiResponse {
  return {
    success: success,
    message: message,
    data: data
  };  
}
