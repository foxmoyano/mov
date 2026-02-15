import { HttpErrorResponse, HttpInterceptorFn } from '@angular/common/http';
import { catchError, throwError } from 'rxjs';
import { isDevMode } from '@angular/core';

export const errorInterceptor: HttpInterceptorFn = (req, next) => {
  return next(req).pipe(
    catchError((error: HttpErrorResponse) => {
      let errorMessage = 'Ocurrió un error inesperado';
      
      // 1. Normalización del error
      const errorPayload = {
        message: error.error?.message || error.message || errorMessage,
        status: error.status,
        code: error.error?.code || 'UNKNOWN_ERROR'
      };

      // 2. Logging opcional en desarrollo
      if (isDevMode()) {
        console.error(`[Error Interceptor] [${req.method}] ${req.url}`, errorPayload);
      }

      // Aquí podrías disparar un servicio de notificaciones (Toast/Alert)
      
      return throwError(() => errorPayload);
    })
  );
};