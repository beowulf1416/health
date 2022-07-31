import { HttpClient, HttpErrorResponse, HttpResponse } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { catchError, map, Observable, of, tap } from 'rxjs';
import { ApiResponse } from 'src/app/classes/api-response';
import { environment } from 'src/environments/environment';

@Injectable({
  providedIn: 'root'
})
export class UserService {

  constructor(
    private http: HttpClient
  ) { }

  handleError(e: HttpErrorResponse) {
    console.error(e);
  }

  authenticate(
    email: string,
    password: string
  ): Observable<boolean> {
    console.log('UserService::authenticate()');
    return this.http.post<ApiResponse>(
      environment.url_base + environment.path_authenticate,
      {
        email: email,
        password: password
      },
      {
        observe: 'response'
      }
    ).pipe(
      tap(r => {
        if (r.headers.has('authorization')) {
          const header_value = r.headers.get('authorization');
          if (header_value != null) {
            const token = header_value.replace('Bearer ', '');
            console.log(token);
            // sessionStorage.setItem(environment.key_session_token, token);
          }
        }
      }),
      // catchError( e => {
      //   console.error(e);
      //   return of(false);
      // })
      map(r => {
        if (r.ok) {
          const api_response = r.body;
          if(api_response?.status == 'success') {
            console.log(api_response?.message);
            return true;
          } else {
            console.error(api_response?.message);
            return false;
          }
        } else {
          console.error(r);
          return false;
        }
      })
    );
  }
}
