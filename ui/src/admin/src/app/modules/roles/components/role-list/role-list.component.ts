import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { ApiResponse } from 'src/app/classes/api-response';
import { TitleService } from 'src/app/services/title.service';
import { RoleService } from '../../services/role.service';

@Component({
  selector: 'app-role-list',
  templateUrl: './role-list.component.html',
  styleUrls: ['./role-list.component.css']
})
export class RoleListComponent implements OnInit {

  pager = {
    items: 10,
    current: 0,
    max: 10
  };

  rolesForm = new FormGroup({
    filter: new FormControl('')
  });

  roles = Array<{
    name: string,
    slug: string
  }>(); 

  constructor(
    private title: TitleService,
    private roles_service: RoleService
  ) { }

  ngOnInit(): void {
    this.title.set_title('Roles');
  }

  get filter() {
    return this.rolesForm.get('filter');
  }

  _fetch_roles() {
    this.roles_service.fetch(
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

    this._fetch_roles();
  }

  page_previous() {
    console.log('page_previous');

    this.pager.current = this.pager.current - 1;
    if (this.pager.current < 0){
      this.pager.current = 0;
    }

    this._fetch_roles();
  }

  page_next() {
    console.log('page_next');

    this.pager.current = this.pager.current + 1;
    if (this.pager.current > this.pager.max) {
      this.pager.current = this.pager.max;
    }

    this._fetch_roles();
  }

  page_last() {
    console.log('//TODO page_last');

    this.pager.current = this.pager.max;

    this._fetch_roles();
  }
  
  page_select(e: Event) {
    console.log('page_select');
    if (e?.target) {
      const value = (e?.target as HTMLSelectElement).value;
      this.pager.current = parseInt(value);

      this._fetch_roles();
    }
  }
}
