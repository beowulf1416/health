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
    this.domain_service.list(
      filter_text + '%',
      10,
      0
    ).subscribe((r: ApiResponse) => {
      if (r.success) {
        console.log(r);
      } else {
        console.error(r);
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

      this.domain_service.list(
        this.filter?.value || '',
        10,
        0
      ).subscribe((r: ApiResponse) => {
        console.log(r);

        this.is_submitting = false;
      });
    }
  }
}
