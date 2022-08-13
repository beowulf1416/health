import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { map, Observable, Subject, tap } from 'rxjs';
import { environment } from 'src/environments/environment';
import { ApiResponse, newApiResponse } from '../classes/api-response';
import { User } from '../classes/user';


// export class User {
//   constructor(
//     private email: string
//   ) {}

//   get is_authenticated(): boolean {
//     return this.email != "";
//   }
// };


export function JSONtoUser(
  json: JSON
): User {
  const sz = JSON.stringify(json);
  const o = JSON.parse(sz);
  const u = o.user as User;
  return u;
}

@Injectable({
  providedIn: 'root'
})
export class UserService {

  // user_subject = new Subject<User>();
  // user$ = this.user_subject.asObservable();

  constructor(
    private http: HttpClient
  ) { }

  get_current_user(): Observable<User> {
    return this.http.post<ApiResponse>(
      environment.url_base + environment.path_user_current,
      {}
    ).pipe(
      map((r: ApiResponse) => {
        // console.log(r.data);

        let u = new User("");

        if (r.data != null && r.data.hasOwnProperty("user")) {
          const udata = (r.data as any)?.user;
          
          u = new User(udata?.email || "");
        }

        return u;
      })
    );
  }

  login(
    email: string,
    password: string
  ): Observable<ApiResponse> {
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
            // console.log(token);
            sessionStorage.setItem(environment.key_session_token, token);
          }
        }
      }),
      map(r => {
        if (r.body == null) {
          return newApiResponse(
            false,
            "unknown error",
            null
          );
        } else {
          return r.body;
        };
      })
    );
  }

  logout() {
    sessionStorage.removeItem(environment.key_session_token);
  }
}
