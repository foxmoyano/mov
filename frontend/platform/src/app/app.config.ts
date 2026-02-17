import { ApplicationConfig, LOCALE_ID, provideBrowserGlobalErrorListeners } from '@angular/core';
import { provideRouter } from '@angular/router';
import { routes } from './app.routes';
import { provideHttpClient, withInterceptors } from '@angular/common/http';
import { errorInterceptor } from './core/interceptors/error.interceptor';
import { providePrimeNG } from 'primeng/config';
import Nora from '@primeuix/themes/nora';

// --- NUEVAS LÍNEAS PARA EL LOCALE ---
import { registerLocaleData } from '@angular/common';
import localeEsCl from '@angular/common/locales/es-CL';

// Registramos físicamente los datos de Chile
registerLocaleData(localeEsCl, 'es-CL');

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideRouter(routes),
    provideHttpClient(),
    providePrimeNG({
      theme: {
        preset: Nora,
        options: {
          darkModeSelector: 'none'
        }
      }
    }),    
    provideHttpClient(withInterceptors([errorInterceptor])),
    { provide: LOCALE_ID, useValue: 'es-CL' }
  ]
};