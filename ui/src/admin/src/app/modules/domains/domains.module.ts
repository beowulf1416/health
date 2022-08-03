import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ReactiveFormsModule } from '@angular/forms';
import { MaterialModule } from 'src/material/material.module';

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
    ReactiveFormsModule,
    MaterialModule,
    DomainsRoutingModule
  ]
})
export class DomainsModule { }
