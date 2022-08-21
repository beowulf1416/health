import { Component, OnInit } from '@angular/core';
import { Observable } from 'rxjs';
import { User } from './classes/user';
import { UserService } from './services/user.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent implements OnInit {

  constructor(
    private user_service: UserService
  ) {}

  ngOnInit(): void {
  }

  get user$(): Observable<User> {
    return this.user_service.user$;
  }
}
