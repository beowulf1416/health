import { Component, OnInit } from '@angular/core';
import { Subject } from 'rxjs';
import { User } from './classes/user';
import { UserService } from './services/user.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent implements OnInit  {
  
  user_subject = new Subject<User>();
  user$ = this.user_subject.asObservable();

  constructor(
    private user_service: UserService
  ) {}

  ngOnInit(): void {
    this.user_service.get_current_user().subscribe((user: User) => {
      this.user_subject.next(user);
    });
  }

}
