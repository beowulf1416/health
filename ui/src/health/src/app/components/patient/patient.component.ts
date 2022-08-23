import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-patient',
  templateUrl: './patient.component.html',
  styleUrls: ['./patient.component.css']
})
export class PatientComponent implements OnInit {

  patientForm = new FormGroup({
    family_name: new FormControl('', []),
    given_name: new FormControl('', []),

    contact_no: new FormControl('', []),
    contact_address: new FormControl('', []),

    gender: new FormControl('', []),
    marital_status: new FormControl('', []),
    ethnicity: new FormControl('', []),

    admission_source: new FormControl('', []),
    current_episode: new FormControl('', []),
    
    birth_date: new FormControl('', []),
    admission_date: new FormControl('', []),
    face_to_face_date: new FormControl('', []),
    soc_date: new FormControl('', []),
    certification_date: new FormControl('', []),
    discharge_date: new FormControl('', []),

    ss_id: new FormControl('', []),
    medicare_id: new FormControl('', []),
    medicaid_id: new FormControl('', []),
    cbsa_id: new FormControl('', []),

    release_of_info: new FormControl('', [])
  });

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

  get family_name() {
    return this.patientForm.get('family_name');
  }

  get given_name() {
    return this.patientForm.get('given_name');
  }

  get address() {
    return this.patientForm.get('contact_address');
  }

  get contact_no() {
    return this.patientForm.get('contact_no');
  }

  get birth_date() {
    return this.patientForm.get('birth_date');
  }

  get gender() {
    return this.patientForm.get('gender');
  }

  get marital_status() {
    return this.patientForm.get('marital_status');
  }

  get ethnicity() {
    return this.patientForm.get('ethnicity');
  }

  get admission_source() {
    return this.patientForm.get('admission_source');
  }

  get admission_date() {
    return this.patientForm.get('admission_date');
  }

  get face_to_face_date() {
    return this.patientForm.get('face_to_face_date');
  }

  get soc_date() {
    return this.patientForm.get('soc_date');
  }

  get current_episode() {
    return this.patientForm.get('current_episode');
  }

  get certification_date() {
    return this.patientForm.get('certification_date');
  }
}
