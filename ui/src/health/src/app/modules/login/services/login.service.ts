import { HttpClient, HttpErrorResponse, HttpResponse } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { catchError, tap, throwError } from 'rxjs';
import { ApiResponse } from 'src/app/classes/api-response';
import { environment } from 'src/environments/environment';

@Injectable({
  providedIn: 'root'
})
export class LoginService {

  constructor(
    private http: HttpClient
  ) { }

  authenticate(
    email: String,
    password: String
  ) {
    this.http.post<ApiResponse>(
      environment.url_base + environment.url_authenticate,
      {
        email: email,
        password: password
      },
      {
        observe: 'response'
      }
    ).pipe(
      tap( o => {
        if (o.headers.has("authorization")) {
          const authorization = o.headers.get("authorization");
          if (authorization != null) {
            const token = authorization?.replace("Bearer", "");
            sessionStorage.setItem(environment.key_session_token, token);
          }
        }
      }),
      catchError(e => this.handleError(e))
    );
  }

  handleError(error: HttpErrorResponse) {
    if (error.status === 0) {
      return throwError({
        status: "error",
        message: "client side error",
        data: {}
      });
    } else {
      return throwError({
        status: "error",
        message: "backend error",
        data: {}
      });
    }
  }
}
