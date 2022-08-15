import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { ApiResponse } from 'src/app/classes/api-response';
import { TitleService } from 'src/app/services/title.service';
import { DomainService } from '../../services/domain.service';

@Component({
  selector: 'app-domain-list',
  templateUrl: './domain-list.component.html',
  styleUrls: ['./domain-list.component.css']
})
export class DomainListComponent implements OnInit {

  filterForm = new FormGroup({
    filter: new FormControl('')
  });

  constructor(
    private title: TitleService,
    private domain_service: DomainService
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

  get filter() {
    return this.filterForm.get('filter');
  }

  submit() {

  }
}
