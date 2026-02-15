import { inject, Injectable, signal, computed } from '@angular/core';
import { VideosApi } from './videos.api';
import { Video, VideosResponse } from './video.model';

@Injectable({ providedIn: 'root' })
export class VideosStore {
  private readonly api = inject(VideosApi);

  // Estado privado (WritableSignals)
  #videos = signal<Video[]>([]);
  #loading = signal<boolean>(false);
  #total = signal<number>(0);

  // Estado público (Signals de solo lectura)
  readonly videos = computed(() => this.#videos());
  readonly loading = computed(() => this.#loading());
  readonly count = computed(() => this.#total());

  loadVideos(page?: number | null, size?: number | null) {
    this.#loading.set(true);
    const params: any = {};
    if (page != null) params.page = page;
    if (size != null) params.size = size;
    this.api.list(params).subscribe({
      next: (res) => {
        // Si la respuesta es un objeto con data y total
        if (res && Array.isArray(res.items)) {
          this.#videos.set(res.items);
          this.#total.set(res.total ?? res.items.length);
        } else if (Array.isArray(res)) {
          // fallback si la API devuelve solo array
          this.#videos.set(res);
          this.#total.set(res.length);
        }
      },
      complete: () => this.#loading.set(false)
    });
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