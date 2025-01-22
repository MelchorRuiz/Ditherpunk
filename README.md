# Ditherpunk

## Question 2

**On obtient un DynamicImage, à quoi correspond ce type?**<br>
Rgba8

**Comment obtenir une image en mode rbg8 à partir de ce DynamicImage?**<br>
```img.to_rgb8();```

## Question 3
**Que se passe-t-il si l’image de départ avait un canal alpha?**<br>
Tous les pixels transparents deviennent noirs, comme le montre l'exemple suivant

| Avant | Après | 
| :-: | :-: |
| ![avant](./img/gato.png) | ![apres](./img/gato_rgb.png) |
