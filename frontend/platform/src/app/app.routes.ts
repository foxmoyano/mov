import { Routes } from '@angular/router';
import { Index } from './features/videos/pages/index/index';

export const routes: Routes = [
  {
    path: '',
    component: Index
  },
  {
    path: '**',
    redirectTo: ''
  }
];