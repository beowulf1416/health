import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { ApiResponse } from 'src/app/classes/api-response';
import { TitleService } from 'src/app/services/title.service';
import { UsersService } from '../../services/users.service';

@Component({
  selector: 'app-users-list',
  templateUrl: './users-list.component.html',
  styleUrls: ['./users-list.component.css']
})
export class UsersListComponent implements OnInit {

  pager = {
    items: 10,
    current: 0,
    max: 10
  };

  usersForm = new FormGroup({
    filter: new FormControl('')
  });

  users = Array<{
    email: string,
    given_name: string,
    family_name: string,
    prefix: string,
    suffix: string
  }>(); 

  constructor(
    private title: TitleService,
    private users_service: UsersService
  ) { }

  ngOnInit(): void {
    this.title.set_title('Users');

    this._fetch_users();
  }

  get filter() {
    return this.usersForm.get('filter');
  }

  _fetch_users() {
    this.users_service.fetch(
      this.filter?.value || '',
      this.pager.items,
      this.pager.current
    ).subscribe((r: ApiResponse) => {
      console.log(r);
    });
  }

  page_first() {
    console.log('page_first');
    this.pager.current = 0;

    this._fetch_users();
  }

  page_previous() {
    console.log('page_previous');

    this.pager.current = this.pager.current - 1;
    if (this.pager.current < 0){
      this.pager.current = 0;
    }

    this._fetch_users();
  }

  page_next() {
    console.log('page_next');

    this.pager.current = this.pager.current + 1;
    if (this.pager.current > this.pager.max) {
      this.pager.current = this.pager.max;
    }

    this._fetch_users();
  }

  page_last() {
    console.log('//TODO page_last');

    this.pager.current = this.pager.max;

    this._fetch_users();
  }
  
  page_select(e: Event) {
    console.log('page_select');
    if (e?.target) {
      const value = (e?.target as HTMLSelectElement).value;
      this.pager.current = parseInt(value);

      this._fetch_users();
    }
  }
}
