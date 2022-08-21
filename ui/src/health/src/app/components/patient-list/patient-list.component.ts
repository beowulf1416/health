import { Component, OnInit } from '@angular/core';
import { FormGroup } from '@angular/forms';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-patient-list',
  templateUrl: './patient-list.component.html',
  styleUrls: ['./patient-list.component.css']
})
export class PatientListComponent implements OnInit {

  patientsForm = new FormGroup({});

  constructor(
    private title: TitleService
  ) {
    this.title.set_title('Patients');
  }

  ngOnInit(): void {
  }

}
