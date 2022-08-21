import { Component, OnInit } from '@angular/core';
import { FormGroup } from '@angular/forms';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-patient',
  templateUrl: './patient.component.html',
  styleUrls: ['./patient.component.css']
})
export class PatientComponent implements OnInit {

  patientForm = new FormGroup({});

  patient = {
    givenName: 'not set',
    familyName: 'not set',
    honorificPrefix: 'not set',
    honorificSuffix: 'not set'
  };

  constructor(
    private title: TitleService
  ) {
    this.title.set_title('Patient');
  }

  ngOnInit(): void {
  }

}
