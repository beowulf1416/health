import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { HomeComponent } from './components/home/home.component';

const routes: Routes = [
  {
    path: 'domains',
    loadChildren: () => import('./modules/domains/domains.module').then( m => m.DomainsModule)
  },
  {
    path: 'roles',
    loadChildren: () => import('./modules/roles/roles.module').then( m => m.RolesModule)
  },
  {
    path: 'users',
    loadChildren: () => import('./modules/users/users.module').then( m => m.UsersModule)
  },
  {
    path: '',
    component: HomeComponent
  }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
