import { Filters } from "../../core/models/filters.model";

export interface Video {
  id: string;
  title: string;
  extension: string;
  size_mb: number;
  published_at: string;
}

// Para videos usamos la base, pero estamos listos para extenderla
// Si mañana necesitas filtrar por 'codec', lo agregas aquí.
export interface VideoFilters extends Filters {
  // extension?: string; (ejemplo de campo extra futuro)
}