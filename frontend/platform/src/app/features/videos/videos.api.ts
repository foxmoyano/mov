import { inject, Injectable } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { Observable } from 'rxjs';
import { Video } from './video.model'; 
import { environment } from '../../../environment/environment';
import { PaginatedResponse } from '../../core/models/pagination.model';

@Injectable({ providedIn: 'root' })
export class VideosApi {
  private readonly http = inject(HttpClient);  
  private readonly baseUrl = `${environment.apiBaseUrl}/videos`;  

  /**
   * Esto hace que la interfaz sea reutilizable.
   */
  list(params?: any): Observable<PaginatedResponse<Video>> {
    let httpParams = new HttpParams();

    if (params) {
      // Un truco más limpio para iterar todos los filtros y paginación
      // sin tener que hacer if por cada campo:
      Object.keys(params).forEach(key => {
        const value = params[key];
        if (value !== null && value !== undefined) {
          httpParams = httpParams.set(key, value.toString());
        }
      });
    }

    return this.http.get<PaginatedResponse<Video>>(this.baseUrl, { params: httpParams });
  }

  // Los métodos get, create, update y delete se mantienen igual
  // ya que operan sobre un solo objeto Video o un void.
  
  get(id: string): Observable<Video> {
    return this.http.get<Video>(`${this.baseUrl}/${id}`);
  }

  create(video: Partial<Video>): Observable<Video> {
    return this.http.post<Video>(this.baseUrl, video);
  }

  update(id: string, video: Partial<Video>): Observable<Video> {
    return this.http.put<Video>(`${this.baseUrl}/${id}`, video);
  }

  delete(id: string): Observable<void> {
    return this.http.delete<void>(`${this.baseUrl}/${id}`);
  }

}