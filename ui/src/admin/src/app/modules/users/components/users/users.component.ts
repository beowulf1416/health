import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { ActivatedRoute } from '@angular/router';
import { ApiResponse } from 'src/app/classes/api-response';
import { TitleService } from 'src/app/services/title.service';
import { UsersService } from '../../services/users.service';

@Component({
  selector: 'app-users',
  templateUrl: './users.component.html',
  styleUrls: ['./users.component.css']
})
export class UsersComponent implements OnInit {

  userForm = new FormGroup({
    given_name: new FormControl('', [
      Validators.required
    ]),
    family_name: new FormControl('', [
      Validators.required
    ]),
    prefix: new FormControl(''. []),
    suffix: new FormControl('', []),
    email: new FormControl('', [
      Validators.required,
      Validators.email
    ])
  });

  constructor(
    private title: TitleService,
    private users: UsersService,
    private route: ActivatedRoute
  ) {
    title.set_title('User');
  }

  ngOnInit(): void {
    const user_slug = this.route.snapshot.paramMap.get('user_slug');
  }


  get email() {
    return this.userForm.get('email');
  }

  get given_name() {
    return this.userForm.get('given_name');
  }

  get family_name() {
    return this.userForm.get('family_name');
  }

  get prefix() {
    return this.userForm.get('prefix');
  }

  get suffix() {
    return this.userForm.get('suffix');
  }

  submit() {
    console.log('UsersComponent::submit()');

    if (this.userForm.valid){
      this.users.add(
        this.userForm.get('given_name')?.value || '',
        this.userForm.get('family_name')?.value || '',
        this.userForm.get('email')?.value || '',
        this.userForm.get('prefix')?.value || '',
        this.userForm.get('suffix')?.value || ''
      ).subscribe((r: ApiResponse) => {
        console.log(r);
      });
    }
  }
}
