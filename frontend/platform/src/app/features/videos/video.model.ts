export interface Video {
  id: string;
  title: string;
  extension: string;
  size_mb: string;
  published_at: string;
}


// Interfaz para la respuesta paginada de videos
export interface VideosResponse {
  items: Video[];
  total: number;
  page: number;
  size: number;
}