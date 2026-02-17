import { DatePipe, DecimalPipe, UpperCasePipe } from '@angular/common';
import { Component, input, output, ViewEncapsulation } from '@angular/core';
import { TableModule, TableLazyLoadEvent } from 'primeng/table';
import { TooltipModule } from 'primeng/tooltip';

@Component({
  selector: 'mov-video-dashboard',
  imports: [
    TableModule,
    TooltipModule,
    DatePipe,
    DecimalPipe,
    UpperCasePipe
  ],
  templateUrl: './dashboard.html',
  styleUrl: './dashboard.css',
  encapsulation: ViewEncapsulation.None
})
export class Dashboard {
  // inputs como signals
  readonly videos = input<any[]>([]);
  readonly loading = input(false);
  readonly total = input(0);
  readonly first = input(0); // Input para sincronizar paginador
  readonly rows = input(10); // Input para sincronizar cantidad de filas

  // output moderno con el tipo correcto
  readonly lazyLoad = output<TableLazyLoadEvent>();

  // Método para manejar el evento
  onLazyLoad(event: TableLazyLoadEvent) {
    this.lazyLoad.emit(event);
  }

}