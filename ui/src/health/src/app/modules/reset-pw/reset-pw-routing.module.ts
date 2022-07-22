import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { ResetPwComponent } from './components/reset-pw/reset-pw.component';

const routes: Routes = [
  { path: '', component: ResetPwComponent }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class ResetPwRoutingModule { }
