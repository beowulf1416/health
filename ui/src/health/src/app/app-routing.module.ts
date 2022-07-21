import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { HomeComponent } from './components/home/home.component';
import { PatientListComponent } from './components/patient-list/patient-list.component';
import { PatientComponent } from './components/patient/patient.component';

const routes: Routes = [
  { path: 'user/login', loadChildren: () => import('./modules/login/login.module').then(m => m.LoginModule)},
  { path: 'patient', component: PatientComponent },
  { path: 'patient-list', component: PatientListComponent },
  { path: '*', component: HomeComponent }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
