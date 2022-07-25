import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { Validator } from 'src/app/classes/validator';
import { TitleService } from 'src/app/services/title.service';
import { LoginService } from '../../services/login.service';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.css']
})
export class LoginComponent implements OnInit {

  loginForm = new FormGroup({
    email: new FormControl('', [
      Validators.required,
      Validators.email
    ]),
    pw: new FormControl('', [
      Validators.minLength(8),
      Validator.pattern(new RegExp('[A-Z]'), { upper: true }),
      Validator.pattern(new RegExp('[0-9]'), { upper: true }),
    ])
  });

  constructor(
    private title: TitleService,
    private login: LoginService
  ) {
    this.title.set_title('Login');
  }

  ngOnInit(): void {
  }

  submit() {
    console.log('LoginComponent::authenticate()');
  }
}
