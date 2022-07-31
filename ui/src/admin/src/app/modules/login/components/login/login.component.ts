import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { TitleService } from 'src/app/services/title.service';
import { UserService } from '../../services/user.service';

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
      Validators.required
    ])
  });

  constructor(
    private title: TitleService,
    private user: UserService
  ) {
    title.set_title('Login');
  }

  ngOnInit(): void {
  }

  get email() {
    return this.loginForm.get('email');
  }

  get password() {
    return this.loginForm.get('pw');
  }

  submit() {
    console.log('LoginComponent::submit()');

    if (this.loginForm.valid) {
      this.user.authenticate(
        this.loginForm.get('email')?.value || '',
        this.loginForm.get('pw')?.value || ''
      ).subscribe(r => {
        console.log(r);
      });
    }
  }
}
