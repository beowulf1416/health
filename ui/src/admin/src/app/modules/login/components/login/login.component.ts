import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { Router } from '@angular/router';
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
    private user: UserService,
    private router: Router
  ) {
    title.set_title('Login');
  }

  ngOnInit(): void {
    if (this.user.isLoggedIn()) {
      this.router.navigate(['']);
    }
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
        if (r.success) {
          this.router.navigate(['']);
        } else {
          this.password?.reset();
        }
      });
    }
  }
}
