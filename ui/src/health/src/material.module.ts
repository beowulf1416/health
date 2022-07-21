import {NgModule} from '@angular/core';

import {MatIconModule} from '@angular/material/icon';
import {MatToolbarModule} from '@angular/material/toolbar';
import {MatSidenavModule} from '@angular/material/sidenav';
import {MatListModule} from '@angular/material/list';

@NgModule({
    exports: [
        MatIconModule,
        MatToolbarModule,
        MatSidenavModule,
        MatListModule
    ]
})
export class MaterialModule {}