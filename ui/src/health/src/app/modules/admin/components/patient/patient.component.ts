import { Component, OnInit } from '@angular/core';
import { FormGroup } from '@angular/forms';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-patient',
  templateUrl: './patient.component.html',
  styleUrls: ['./patient.component.css']
})
export class PatientComponent implements OnInit {

  patientForm = new FormGroup({
    
  });

  constructor(
    private title: TitleService
  ) {
    this.title.set_title("Patient");
  }

  ngOnInit(): void {
  }

}
