import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { UsersRoutingModule } from './users-routing.module';
import { UsersComponent } from './components/users/users.component';
import { UsersListComponent } from './components/users-list/users-list.component';


@NgModule({
  declarations: [
    UsersComponent,
    UsersListComponent
  ],
  imports: [
    CommonModule,
    UsersRoutingModule
  ]
})
export class UsersModule { }
