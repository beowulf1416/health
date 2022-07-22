import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { AdminHomeComponent } from './components/admin-home/admin-home.component';
import { PatientListComponent } from './components/patient-list/patient-list.component';
import { PatientComponent } from './components/patient/patient.component';
import { UserListComponent } from './components/user-list/user-list.component';
import { UserComponent } from './components/user/user.component';

const routes: Routes = [
  { path: '', component: AdminHomeComponent },
  { path: 'user-list', component: UserListComponent },
  { path: 'user/add', component: UserComponent },
  { path: 'user/edit', component: UserComponent },
  { path: 'user/view', component: UserComponent },
  { path: 'patient-list', component: PatientListComponent },
  { path: 'patient/add', component: PatientComponent },
  { path: 'patient/edit', component: PatientComponent },
  { path: 'patient/view', component: PatientComponent },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class AdminRoutingModule { }
