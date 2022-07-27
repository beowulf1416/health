import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { DomainsRoutingModule } from './domains-routing.module';
import { DomainComponent } from './components/domain/domain.component';
import { DomainListComponent } from './components/domain-list/domain-list.component';


@NgModule({
  declarations: [
    DomainComponent,
    DomainListComponent
  ],
  imports: [
    CommonModule,
    DomainsRoutingModule
  ]
})
export class DomainsModule { }
