import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { ApiResponse } from 'src/app/classes/api-response';

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
    slug: string,
    email: string
  ) /*: Observable<ApiResponse>*/ {
    console.log('//TODO: UsersService::add');
    console.log({
      given_name: given_name,
      family_name: family_name,
      slug: slug,
      email: email
    });
  }
}
