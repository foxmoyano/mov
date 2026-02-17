// Para el estado del componente (PrimeNG)
export interface PaginationState {
  page: number;
  rows: number;
  first: number;
}

// Para la respuesta que viene desde la api
export interface PaginatedResponse<T> {
  items: T[];
  total: number;
  page: number;
  size: number;
}