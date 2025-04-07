# Ray Tracer

This program is  a basic Ray Tracer written in Rust.
For now, it allows you to place spheres in 3D space and render them, with lights.

## Changelog

### v0.2.0

- Ajout Parser et Tokeniser
- Ajout de tests de performance automatique
- Amélioration de la gestion des Angles

### v0.1.1

- Suppression de fichiers inutilisés.

#### Bugfixes

- [#1] (et [#2]) : Problème de self intersection si jamais le point d'intersection était légérement dans la sphère. 

### v0.1.0

- Basic camera, sphreres and lights.
- 3D Geometry tools (Vect3, Quat, Transform).
- Image rendered to ppm file.
- Rotation is included in objects but not fully used yet.
