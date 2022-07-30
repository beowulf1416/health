import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class UserService {

  constructor(
    private http: HttpClient
  ) { }

  authenticate(
    email: string,
    password: string
  ) {
    console.log('UserService::authenticate()');
  }
}
