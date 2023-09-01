# EXCUSAS EXCUSAS API REST
Sencilla API Rest de excusas a situaciones típicas dependiendo de la categoría a la que pertenezcan:
- **Compromisos y Eventos Sociales**: situaciones relacionadas con la imposibilidad de asistir a eventos, actividades sociales, celebraciones, bodas, funerales y compromisos sociales en general
- **Tareas y Responsabilidades**: incluye situaciones en las que no se puede cumplir con tareas, responsabilidades laborales, presentaciones importantes, plazos, y donde se necesitan excusas relacionadas con el trabajo y las responsabilidades.
- **Limitaciones y Habilidades Personales**: situaciones en las que la falta de habilidades, conocimientos o memoria juegan un papel, como no poder dar seguimiento a tareas, no cumplir con expectativas, olvidar detalles importantes, etc.
- **Cambios de Circunstancias y Prioridades**: situaciones que involucran cambios inesperados, prioridades modificadas, imposibilidad de colaborar o asumir nuevas responsabilidades debido a cambios en las circunstancias.
- **Dificultades Externas y Limitaciones Técnicas**: contiene situaciones donde factores externos como problemas técnicos, falta de información, distracciones o límites de tiempo afectan la capacidad de cumplir con compromisos o tareas.
- **Situaciones Específicas**: agrupa situaciones que no encajan perfectamente en las otras categorías, como no poder responder mensajes o llamadas, cancelar planes, no poder asumir responsabilidades voluntarias, etc.
           
Los endpoints son:
- GET: /categorias
- GET /categorias/{categoria}/situaciones
- GET /situaciones/{situacion}/excusas

El contenido ha sido generado por Chat GPT y está sistematizado en src/ bbdd.json. Se ha utilizado com framework actix-web

No tiene mayor propósito ni finalidad que continuar con mis primeros pasos en Rust.