valid_triangle(A, B, C):-
    A > 0,
    B > 0,
    C > 0,
    A + B >= C,
    A + C >= B,
    B + C >= A.

triangle(X, X, X, "equilateral"):-
    valid_triangle(X, X, X).

triangle(X, Y, Z, "isosceles"):-
    valid_triangle(X, Y, Z),
    (
        X =:= Y ;
        X =:= Z ;
        Y =:= Z
    ).

triangle(X, Y, Z, "scalene"):-
    valid_triangle(X, Y, Z),
    X \== Y,
    X \== Z,
    Y \== Z.