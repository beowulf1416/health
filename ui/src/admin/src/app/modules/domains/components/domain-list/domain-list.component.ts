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

  filterForm = new FormGroup({
    filter: new FormControl('')
  });

  domains = Array<{
    id: string,
    active: boolean,
    name: string,
    slug: string
  }>();;

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
      10,
      0
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
    return this.filterForm.get('filter');
  }

  submit() {
    if (!this.is_submitting) {
      this.is_submitting = true;

      // this.domain_service.list(
      //   this.filter?.value || '',
      //   10,
      //   0
      // ).subscribe((r: ApiResponse) => {
      //   // console.log(r);
      //   if (r.success) {
      //     if (r.data?.hasOwnProperty('domains')) {
      //       const o: any = r.data;
      //       this.domains = o?.domains;
      //     } else {
      //       this.domains = [];
      //     }
      //   }

      this._fetch_domains();

        this.is_submitting = false;
      // });
    }
  }
}
