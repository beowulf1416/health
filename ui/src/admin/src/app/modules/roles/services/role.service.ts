import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { ApiResponse } from 'src/app/classes/api-response';
import { environment } from 'src/environments/environment';

import {v4 as uuidv4} from 'uuid';


@Injectable({
  providedIn: 'root'
})
export class RoleService {

  constructor(
    private http: HttpClient
  ) { }

  add(
    domain_id: string,
    name: string,
    slug: string
  ): Observable<ApiResponse> {
    const new_id = uuidv4();

    return this.http.post<ApiResponse>(
      environment.url_base + environment.path_role_add,
      {
        id: new_id,
        domain_id: domain_id,
        name: name,
        slug: slug
      }
    );
  }

  list(
    filter: string,
    items: number,
    page: number
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.url_base + environment.path_role_list,
      {
        filter: filter,
        items: items,
        page: page
      }
    );
  }
}
