import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { Router } from '@angular/router';
import { Subject } from 'rxjs';
import { User } from 'src/app/classes/user';
import { TitleService } from 'src/app/services/title.service';
import { UserService } from 'src/app/services/user.service';
// import { UserService } from '../../services/user.service';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.css']
})
export class LoginComponent implements OnInit {

  // user_subject = new Subject<User>();
  // user$ = this.user_subject.asObservable();

  is_submitting = false;

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
    private user_service: UserService,
    private router: Router
  ) {
    title.set_title('Login');
  }

  ngOnInit(): void {
    // if (this.user.isLoggedIn()) {
    //   this.router.navigate(['']);
    // }

    // this.user_service.current_user.subscribe((user: User) => {
    //   if (user.is_logged_in) {
    //     this.router.navigate(['']);
    //   } else {
    //     this.user_subject.next(user);
    //   }
    // });
  }

  get email() {
    return this.loginForm.get('email');
  }

  get password() {
    return this.loginForm.get('pw');
  }

  submit() {
    if (this.loginForm.valid) {
      this.is_submitting = true;

      this.user_service.login(
        this.loginForm.get('email')?.value || '',
        this.loginForm.get('pw')?.value || ''
      ).subscribe(r => {

        console.log(r);

        // if (r.success) {
        //   this.router.navigate(['']);
        // } else {
        //   this.password?.reset();
        // }

        this.is_submitting = false;
      });
    }
  }
}
