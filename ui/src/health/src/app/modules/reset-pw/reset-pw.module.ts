import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { ResetPwRoutingModule } from './reset-pw-routing.module';
import { ResetPwComponent } from './components/reset-pw/reset-pw.component';


@NgModule({
  declarations: [
    ResetPwComponent
  ],
  imports: [
    CommonModule,
    ResetPwRoutingModule
  ]
})
export class ResetPwModule { }
