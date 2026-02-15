import { Component, inject } from '@angular/core';
import { VideosStore } from '../../videos.store';
import { Dashboard } from '../../components/dashboard/dashboard';
import { TableLazyLoadEvent } from 'primeng/table';
import { ButtonModule } from 'primeng/button';
import { CardModule } from 'primeng/card';

@Component({
  selector: 'mov-index',
  imports: [
    Dashboard,
    ButtonModule,
    CardModule
  ],
  templateUrl: './index.html',
  styleUrl: './index.css',
})
export class Index {
  // Inyección moderna con inject()
  readonly store = inject(VideosStore);

  // Signals públicas accesibles en el template
  readonly videos = this.store.videos;
  readonly loading = this.store.loading;
  readonly count = this.store.count;

  onLazyLoad(event: TableLazyLoadEvent) {
    // PrimeNG dispara este evento automáticamente al inicializar
    const first = event.first ?? 0;
    const rows  = event.rows ?? 10;
    const page  = Math.floor(first / rows);

    console.log('Lazy load triggered:', { page, rows, first });
    
    // Cargar videos - se puede extender para soportar paginación backend
    this.store.loadVideos(page, rows);
  }  

}