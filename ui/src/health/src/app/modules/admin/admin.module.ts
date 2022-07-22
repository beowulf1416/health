import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { AdminRoutingModule } from './admin-routing.module';
import { AdminHomeComponent } from './components/admin-home/admin-home.component';
import { UserListComponent } from './components/user-list/user-list.component';
import { UserComponent } from './components/user/user.component';
import { PatientListComponent } from './components/patient-list/patient-list.component';
import { PatientComponent } from './components/patient/patient.component';


@NgModule({
  declarations: [
    AdminHomeComponent,
    UserListComponent,
    UserComponent,
    PatientListComponent,
    PatientComponent
  ],
  imports: [
    CommonModule,
    AdminRoutingModule
  ]
})
export class AdminModule { }
