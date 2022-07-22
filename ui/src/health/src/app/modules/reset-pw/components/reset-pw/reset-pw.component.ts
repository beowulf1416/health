import { Component, OnInit } from '@angular/core';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-reset-pw',
  templateUrl: './reset-pw.component.html',
  styleUrls: ['./reset-pw.component.css']
})
export class ResetPwComponent implements OnInit {

  constructor(
    private title: TitleService
  ) {
    this.title.set_title('Reset Password');
  }

  ngOnInit(): void {
  }

}
