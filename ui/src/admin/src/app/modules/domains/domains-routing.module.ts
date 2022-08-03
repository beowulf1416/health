import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { DomainListComponent } from './components/domain-list/domain-list.component';
import { DomainComponent } from './components/domain/domain.component';

const routes: Routes = [
  {
    path: '',
    component: DomainListComponent
  },
  {
    path: 'view',
    component: DomainComponent
  },
  {
    path: 'add',
    component: DomainComponent
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class DomainsRoutingModule { }
