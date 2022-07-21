import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'app-patient',
  templateUrl: './patient.component.html',
  styleUrls: ['./patient.component.css']
})
export class PatientComponent implements OnInit {

  patient = {
    givenName: 'not set',
    familyName: 'not set',
    honorificPrefix: 'not set',
    honorificSuffix: 'not set'
  };

  constructor() { }

  ngOnInit(): void {
  }

}
