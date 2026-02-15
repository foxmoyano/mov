import { Component, input, output } from '@angular/core';
import { TableModule, TableLazyLoadEvent } from 'primeng/table';

@Component({
  selector: 'mov-dashboard',
  imports: [
    TableModule 
  ],
  templateUrl: './dashboard.html',
  styleUrl: './dashboard.css',
})
export class Dashboard {
  // inputs como signals
  readonly videos = input<any[]>([]);
  readonly loading = input(false);
  readonly total = input(0);

  // output moderno con el tipo correcto
  readonly lazyLoad = output<TableLazyLoadEvent>();

  // Método para manejar el evento
  onLazyLoad(event: TableLazyLoadEvent) {
    this.lazyLoad.emit(event);
  }

}