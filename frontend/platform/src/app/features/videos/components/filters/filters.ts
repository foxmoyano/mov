import { Component, output, ViewEncapsulation } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { ButtonModule } from 'primeng/button';
import { InputTextModule } from 'primeng/inputtext';
import { InputNumberModule } from 'primeng/inputnumber';
import { CardModule } from 'primeng/card';

@Component({
  selector: 'mov-video-filters',
  standalone: true,
  imports: [CommonModule, FormsModule, ButtonModule, InputTextModule, InputNumberModule, CardModule],
  templateUrl: './filters.html',
  styleUrls: ['./filters.css'],
  encapsulation: ViewEncapsulation.None
})
export class VideoFilters {
  // Modelos para los inputs
  title: string = '';
  year: number | null = null;

  // Evento que notificará al padre los filtros aplicados
  filterChange = output<{ title: string; year: number | null }>();

  onSearch() {
    this.filterChange.emit({
      title: this.title,
      year: this.year
    });
  }

  onClear() {
    this.title = '';
    this.year = null;
    this.onSearch(); // Notifica la limpieza para recargar la lista completa
  }
}