import { HttpClient, HttpResponse } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { BehaviorSubject, filter, map, Observable, Subject, switchMap, tap } from 'rxjs';
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


// https://github.com/gothinkster/angular-realworld-example-app/blob/master/src/app/core/services/user.service.ts

// export function JSONtoUser(
//   json: JSON
// ): User {
//   const sz = JSON.stringify(json);
//   const o = JSON.parse(sz);
//   const u = o.user as User;
//   return u;
// }

@Injectable({
  providedIn: 'root'
})
export class UserService {

  user_subject = new BehaviorSubject<User>(new User(''));
  user$ = this.user_subject.asObservable();

  constructor(
    private http: HttpClient
  ) { 
    const token = sessionStorage.getItem(environment.key_session_token);
    if (token != null) {
      this._get_current_user().subscribe((r: ApiResponse) => {
        console.log(r);
      });
    }
  }

  get current_user(): Observable<User> {
    return this.user$;
  }

  _get_current_user(): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.url_base + environment.path_user_current,
      {}
    ).pipe(
      filter((r: ApiResponse) => r.success === true),
      tap((r: ApiResponse) => {
        
        const sz = JSON.stringify(r.data);
        const o = JSON.parse(sz);
        const u = o.user;

        this.user_subject.next(new User(u?.email));
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
      filter((r: HttpResponse<ApiResponse>) => { console.log(r); return r.body?.success === true; }),
      switchMap((r: any) => {
        console.log(r);

        // return this.http.post<ApiResponse>(
        //   environment.url_base + environment.path_user_current,
        //   {}
        // ).pipe(
        //   filter((r: ApiResponse) => r.success === true),
        //   tap((r: ApiResponse) => {
            
        //     const sz = JSON.stringify(r.data);
        //     const o = JSON.parse(sz);
        //     const u = o.user;

        //     this.user_subject.next(new User(u?.email));
        //   })
        // );

        return this._get_current_user();
      })
    );
  }

  logout() {
    sessionStorage.removeItem(environment.key_session_token);
  }
}
