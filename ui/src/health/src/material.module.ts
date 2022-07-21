import {NgModule} from '@angular/core';

import {MatIconModule} from '@angular/material/icon';
import {MatToolbarModule} from '@angular/material/toolbar';
import {MatSidenavModule} from '@angular/material/sidenav';
import {MatListModule} from '@angular/material/list';
import {MatExpansionModule} from '@angular/material/expansion';

@NgModule({
    exports: [
        MatIconModule,
        MatToolbarModule,
        MatSidenavModule,
        MatListModule,
        MatExpansionModule
    ]
})
export class MaterialModule {}