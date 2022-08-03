import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';

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

  constructor() { }

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
  }
}
