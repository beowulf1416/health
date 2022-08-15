import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { ActivatedRoute, Router } from '@angular/router';
import { ApiResponse } from 'src/app/classes/api-response';
import { TitleService } from 'src/app/services/title.service';
import { DomainService } from '../../services/domain.service';

@Component({
  selector: 'app-domain',
  templateUrl: './domain.component.html',
  styleUrls: ['./domain.component.css']
})
export class DomainComponent implements OnInit {

  is_submitting = false;

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
    private domain_service: DomainService,
    private route: ActivatedRoute,
    private router: Router
  ) { 
    title.set_title('Domain');
  }

  ngOnInit(): void {
    const slug = this.route.snapshot.paramMap.get('slug');

    // this.domain_service.get()
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
    if (this.domainForm.valid) {
      this.is_submitting = true;

      this.domain_service.add(
        this.domainForm.get('name')?.value || '',
        this.domainForm.get('slug')?.value || ''
      ).subscribe((r: ApiResponse) => {
        // console.log(r);
        if (r.success) {
          this.router.navigate(["list"]);
        } else {
          console.error(r.message);
        }

        this.is_submitting = false;
      });
    }
  }
}
