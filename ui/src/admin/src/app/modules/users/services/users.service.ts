import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { ApiResponse } from 'src/app/classes/api-response';
import { environment } from 'src/environments/environment';

import {v4 as uuidv4} from 'uuid';

@Injectable({
  providedIn: 'root'
})
export class UsersService {

  constructor(
    private http: HttpClient
  ) { }

  add(
    given_name: string,
    family_name: string,
    email: string,
    prefix: string,
    suffix: string
  ) : Observable<ApiResponse> {
    console.log('//TODO: UsersService::add');
    const new_id = uuidv4();

    return this.http.post<ApiResponse>(
      environment.url_base + environment.path_user_add,
      {
        id: new_id,
        email: email,
        given_name: given_name,
        family_name: family_name,
        prefix: prefix,
        suffix: suffix
      }
    );
  }

  fetch(
    filter: string,
    items: number,
    page: number
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.url_base + environment.path_user_fetch,
      {
        filter: filter,
        items: items,
        page: page
      }
    );
  }
}
