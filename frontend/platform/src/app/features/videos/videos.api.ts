import { inject, Injectable } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { Observable } from 'rxjs';
import { Video, VideosResponse } from './video.model'; // Tu interfaz
import { environment } from '../../../environment/environment';

@Injectable({ providedIn: 'root' })
export class VideosApi {
  private readonly http = inject(HttpClient);  
  private readonly baseUrl = `${environment.apiBaseUrl}/videos`;  

  list(params?: { page?: number; size?: number }): Observable<VideosResponse> {
    let httpParams = new HttpParams();
    if (params) {
      if (params.page != null) httpParams = httpParams.set('page', params.page.toString());
      if (params.size != null) httpParams = httpParams.set('size', params.size.toString());
    }
    return this.http.get<VideosResponse>(this.baseUrl, { params: httpParams });
  }

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