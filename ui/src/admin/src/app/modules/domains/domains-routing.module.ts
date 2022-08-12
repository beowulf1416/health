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
    path: 'add',
    component: DomainComponent
  },
  {
    path: 'view/:slug',
    component: DomainComponent
  },
  {
    path: 'edit/:slug',
    component: DomainComponent
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class DomainsRoutingModule { }
