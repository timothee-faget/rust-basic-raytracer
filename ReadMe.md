# Ray Tracer

This program is  a basic Ray Tracer written in Rust.
For now, it allows you to place spheres in 3D space and render them, with lights.

## Changelog

### v0.4.0

- Ajout de a gestion de plans
- Ajout de la gestion de triangles
- Prise en compte de la distance dans le calcul de la lumière ambiante
- Prise en compte de la distance dans le calcul de la lumière diffuse
- Ajout de tests
- Réorganisation dans la boucle de rendu


#### Bugfixes

- [#5] : Zone d'ombre qui apparaissent avec plusiseurs plans. On prenait en compte des intersections plus loins que la lumière. 



### v0.3.0

- Ajout d'une structure matériau. Prise en compte dans le parser.
- Prise en charge des parties ambiante et spéculaire du modèle de Phong.
- Ajout d'un test de la vitesse de sauvegarde.


#### Bugfixes

- [#4] : Démarcation en tangence d'une sphère. Il y avait des couleurs négative, elles sont maintenant clampées. 


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
