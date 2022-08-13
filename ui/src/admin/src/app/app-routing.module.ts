import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { HomeComponent } from './components/home/home.component';
import { PageNotFoundComponent } from './components/page-not-found/page-not-found.component';

const routes: Routes = [
  {
    path: 'user/login',
    loadChildren: () => import('./modules/login/login.module').then( m => m.LoginModule)
  },
  {
    path: 'user/logout',
    loadChildren: () => import('./modules/logout/logout.module').then( m => m.LogoutModule)
  },
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
  },
  {
    path: '**',
    component: PageNotFoundComponent
  }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
