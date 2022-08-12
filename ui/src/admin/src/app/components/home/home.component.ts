import { Component, OnInit } from '@angular/core';
import { ApiResponse } from 'src/app/classes/api-response';
import { TitleService } from 'src/app/services/title.service';
import { UserService } from 'src/app/services/user.service';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.css']
})
export class HomeComponent implements OnInit {

  constructor(
    private title: TitleService,
    private user_service: UserService
  ) { }

  ngOnInit(): void {
    if (this.user_service.is_logged_in()) {
      this.user_service.get_current_user().subscribe((r: ApiResponse) => {
        console.log(r);
      });
    }
  }

}
