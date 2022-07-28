import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { ActivatedRoute } from '@angular/router';
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
    slug: new FormControl('', [
      Validators.required
    ]),
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

  submit() {
    console.log('UsersComponent::submit()');

    if (this.userForm.valid){
      this.users.add(
        this.userForm.get('given_name')?.value || '',
        this.userForm.get('family_name')?.value || '',
        this.userForm.get('slug')?.value || '',
        this.userForm.get('email')?.value || ''
      );
    }
  }
}
