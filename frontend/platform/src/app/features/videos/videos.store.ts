import { inject, Injectable, signal, computed } from '@angular/core';
import { VideosApi } from './videos.api';
import { Video } from './video.model';
import { PaginationState } from '../../core/models/pagination.model';
import { Filters } from '../../core/models/filters.model';

@Injectable({ providedIn: 'root' })
export class VideosStore {
  private readonly api = inject(VideosApi);

  // Estado privado con objetos estructurados
  readonly #filters = signal<Filters>({ title: '', year: null });
  readonly #pagination = signal<PaginationState>({ page: 0, rows: 10, first: 0 });  

  // Estado privado (WritableSignals)
  readonly #videos = signal<Video[]>([]);
  readonly #loading = signal<boolean>(false);
  readonly #total = signal<number>(0);

  // Estado público (Signals de solo lectura)
  readonly videos = computed(() => this.#videos());
  readonly loading = computed(() => this.#loading());
  readonly count = computed(() => this.#total());
  readonly pagination = computed(() => this.#pagination());

/**
   * REFACTOR: loadVideos ya NO recibe argumentos. 
   * Lee directamente las señales internas.
   */
  loadVideos() {
    this.#loading.set(true);

    // Extraemos los valores actuales de las señales
    const currentFilters = this.#filters();
    const currentPagination = this.#pagination();

    // Construimos el objeto final para la API
    const params = {
      page: currentPagination.page,
      size: currentPagination.rows,
      title: currentFilters.title,
      year: currentFilters.year
    };

    this.api.list(params).subscribe({
      next: (res) => {
        // Tu lógica de seteo de videos y total se mantiene igual
        this.#videos.set(res.items);
        this.#total.set(res.total);
      },
      error: () => this.#loading.set(false), // Importante manejar el error
      complete: () => this.#loading.set(false)
    });
  }

  updateFilters(newFilters: Filters) {
    this.#filters.set(newFilters);
    // Reset de página al filtrar
    this.#pagination.update(p => ({ ...p, page: 0, first: 0 })); 
    this.loadVideos(); // Ahora carga con los filtros nuevos y pág 0
  }

  updatePagination(newState: PaginationState) {
    this.#pagination.set(newState);
    this.loadVideos(); // Ahora carga con los filtros que ya estaban guardados
  }

  addVideo(newVideo: Partial<Video>) {
    this.api.create(newVideo).subscribe(video => {
      this.#videos.update(list => [...list, video]);
    });
  }

  removeVideo(id: string) {
    this.api.delete(id).subscribe(() => {
      this.#videos.update(list => list.filter(v => v.id !== id));
    });
  }
  
}