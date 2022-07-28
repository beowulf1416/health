import { Injectable } from '@angular/core';
import { Title } from '@angular/platform-browser';

@Injectable({
  providedIn: 'root'
})
export class TitleService {

  constructor(
    private title: Title
  ) { }

  set_title(title: String) {
    this.title.setTitle('Admin - ' + title);
  }
}
