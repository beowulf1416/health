import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { environment } from 'src/environments/environment';
import { ApiResponse } from '../classes/api-response';

@Injectable({
  providedIn: 'root'
})
export class UserService {

  constructor(
    private http: HttpClient
  ) { }

  is_logged_in(): boolean {
    return sessionStorage.getItem(environment.key_session_token) != null;
  }

  get_current_user(): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.url_base + environment.path_user_current,
      {}
    );
  }
}
