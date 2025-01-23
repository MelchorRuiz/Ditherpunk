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

## Question 9
**Comment calculer la distance entre deux couleurs?**<br>
J'ai utilisé une méthode mathématique qui mesure la différence entre les deux couleurs dans l'espace colorimétrique tridimensionnel, appelée **distance euclidienne**.

Formule de la distance euclidienne<br>
Si les deux couleurs sont représentées par :
- ( C_1 = (R_1, G_1, B_1) )
- ( C_2 = (R_2, G_2, B_2) )

Alors, la distance \( D \) entre ces deux couleurs est donnée par :

D = √ (R_2 - R_1)^2 + (G_2 - G_1)^2 + (B_2 - B_1)^2

## Question 11
**comportement avec une palette vide**<br>
Le programme renvoie une erreur si le nombre de couleurs à utiliser est inférieur ou égal à 0 ou s'il est supérieur à 8.

## Question 17
**comment l'erreur commise dans chaque pixel est représentée et distribuée.**<br>
asd