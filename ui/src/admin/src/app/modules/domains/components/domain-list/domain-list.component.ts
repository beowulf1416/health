import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { Observable } from 'rxjs';
import { ApiResponse } from 'src/app/classes/api-response';
import { User } from 'src/app/classes/user';
import { TitleService } from 'src/app/services/title.service';
import { UserService } from 'src/app/services/user.service';
import { DomainService } from '../../services/domain.service';

@Component({
  selector: 'app-domain-list',
  templateUrl: './domain-list.component.html',
  styleUrls: ['./domain-list.component.css']
})
export class DomainListComponent implements OnInit {

  is_submitting = false;

  pager = {
    items: 10,
    current: 0,
    max: 10
  };

  domainsForm = new FormGroup({
    filter: new FormControl('')
  });

  domains = Array<{
    id: string,
    active: boolean,
    name: string,
    slug: string
  }>();

  constructor(
    private title: TitleService,
    private domain_service: DomainService,
    private user_service: UserService
  ) { }

  ngOnInit(): void {
    this.title.set_title('Domains');

    this._fetch_domains();
  }

  _fetch_domains() {
    const filter_text = this.filter?.value || '';
    this.domain_service.fetch(
      filter_text + '%',
      this.pager.items,
      this.pager.current
    ).subscribe((r: ApiResponse) => {
      if (r.success) {
        if (r.data?.hasOwnProperty('domains')) {
          const o: any = r.data;
          const d: Array<{
            id: string,
            active: boolean,
            name: string,
            slug: string
          }> = o?.domains;
          this.domains = d;
        } else {
          this.domains = [];
        }
      }
    });
  }

  get user$(): Observable<User> {
    return this.user_service.user$;
  }

  get filter() {
    return this.domainsForm.get('filter');
  }

  submit() {
    if (!this.is_submitting) {
      this.is_submitting = true;

      this._fetch_domains();

      this.is_submitting = false;
    }
  }

  page_first() {
    console.log('page_first');
    this.pager.current = 0;

    this._fetch_domains();
  }

  page_previous() {
    console.log('page_previous');

    this.pager.current = this.pager.current - 1;
    if (this.pager.current < 0){
      this.pager.current = 0;
    }

    this._fetch_domains();
  }

  page_next() {
    console.log('page_next');

    this.pager.current = this.pager.current + 1;
    if (this.pager.current > this.pager.max) {
      this.pager.current = this.pager.max;
    }

    this._fetch_domains();
  }

  page_last() {
    console.log('//TODO page_last');

    this.pager.current = this.pager.max;

    this._fetch_domains();
  }
  
  page_select(e: Event) {
    console.log('page_select');
    // console.log(e);
    // console.log(v);

    if (e?.target) {
      const value = (e?.target as HTMLSelectElement).value;
      this.pager.current = parseInt(value);

      this._fetch_domains();
    }
  }
}
