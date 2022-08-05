import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { ApiResponse } from 'src/app/classes/api-response';
import { TitleService } from 'src/app/services/title.service';
import { DomainService } from '../../services/domain.service';

@Component({
  selector: 'app-domain',
  templateUrl: './domain.component.html',
  styleUrls: ['./domain.component.css']
})
export class DomainComponent implements OnInit {

  domainForm = new FormGroup({
    name: new FormControl('', [
      Validators.required
    ]),
    slug: new FormControl('', [
      Validators.required
    ]),
    active: new FormControl('', [])
  });

  constructor(
    private title: TitleService,
    private domain_service: DomainService
  ) { 
    title.set_title('Domain');
  }

  ngOnInit(): void {
  }

  get name() {
    return this.domainForm?.get('name');
  }

  get slug() {
    return this.domainForm?.get('slug');
  }

  get active() {
    return this.domainForm.get?.('active');
  }
 
  submit() {
    console.log('DomainComponent::submit()');

    if (this.domainForm.valid) {
      this.domain_service.add(
        this.domainForm.get('name')?.value || '',
        this.domainForm.get('slug')?.value || ''
      ).subscribe((r: ApiResponse) => {
        console.log(r);
      });
    }
  }
}
