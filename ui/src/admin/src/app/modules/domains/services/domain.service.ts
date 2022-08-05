import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { ApiResponse } from 'src/app/classes/api-response';
import { environment } from 'src/environments/environment';

import {v4 as uuidv4} from 'uuid';

@Injectable({
  providedIn: 'root'
})
export class DomainService {

  constructor(
    private http: HttpClient
  ) { }

  add(
    name: string,
    slug: string
  ): Observable<ApiResponse> {
    console.log('DomainService::add()');

    const new_id = uuidv4();

    return this.http.post<ApiResponse>(
      environment.url_base + environment.path_domain_add,
      {
        id: new_id,
        name: name,
        slug: slug
      }
    );
  }
}
