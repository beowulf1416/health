import { Component, OnInit } from '@angular/core';
import { Observable, Subject } from 'rxjs';
import { ApiResponse } from 'src/app/classes/api-response';
import { User } from 'src/app/classes/user';
import { TitleService } from 'src/app/services/title.service';
import { UserService } from 'src/app/services/user.service';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.css']
})
export class HomeComponent implements OnInit {

  user_subject = new Subject<User>();
  user$ = this.user_subject.asObservable();

  constructor(
    private title: TitleService,
    private user_service: UserService
  ) { }

  ngOnInit(): void {
    this.user_service.current_user.subscribe((user: User) => {
      this.user_subject.next(user);
    });
  }

}
