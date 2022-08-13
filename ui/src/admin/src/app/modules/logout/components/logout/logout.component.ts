import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { TitleService } from 'src/app/services/title.service';
import { UserService } from 'src/app/services/user.service';

@Component({
  selector: 'app-logout',
  templateUrl: './logout.component.html',
  styleUrls: ['./logout.component.css']
})
export class LogoutComponent implements OnInit {

  constructor(
    private title: TitleService,
    private user_service: UserService,
    private router: Router
  ) { }

  ngOnInit(): void {
    this.title.set_title('Logout');

    setInterval(() => {
      this.user_service.logout();
      this.router.navigate(['']);
    }, 3000);
  }

}
