import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { patternValidator } from 'src/app/classes/pattern-validator';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-role',
  templateUrl: './role.component.html',
  styleUrls: ['./role.component.css']
})
export class RoleComponent implements OnInit {

  roleForm = new FormGroup({
    domain: new FormControl('', [
      Validators.required
    ]),
    name: new FormControl('', [
      Validators.required
    ]),
    slug: new FormControl('', [
      Validators.required,
      patternValidator(new RegExp('[0-9a-z]'), { slug: true })
    ])
  });

  constructor(
    private title: TitleService
  ) {
    this.title.set_title('Role');
  }

  ngOnInit(): void {
  }

  get domain() {
    return this.roleForm.get('domain');
  }

  get name() {
    return this.roleForm.get('name');
  }

  get slug() {
    return this.roleForm.get('slug');
  }

  submit() {
    console.log('RoleComponent::submit()');

    if (this.roleForm.valid) {
      console.log("//TODO RoleComponent::submit()");
    }
  }
}
