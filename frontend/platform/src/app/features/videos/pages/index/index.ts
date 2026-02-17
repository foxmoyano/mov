import { Component, inject } from '@angular/core';
import { VideosStore } from '../../videos.store';
import { Dashboard } from '../../components/dashboard/dashboard';
import { TableLazyLoadEvent } from 'primeng/table';
import { ButtonModule } from 'primeng/button';
import { CardModule } from 'primeng/card';
import { VideoFilters } from "../../components/filters/filters";

@Component({
  selector: 'mov-video-index',
  imports: [    
    ButtonModule,
    CardModule,
    Dashboard,
    VideoFilters
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
    // PrimeNG dispara este evento automáticamente al inicializar o paginar
    const first = event.first ?? 0;
    const rows  = event.rows ?? 10;
    const page  = Math.floor(first / rows);
    // Actualiza el estado de paginación en el store
    this.store.updatePagination({ page, rows, first });
  }

  onFilter(filters: any) {
    // Actualiza los filtros en el store y resetea la paginación
    this.store.updateFilters(filters);
  }

}