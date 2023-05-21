isogram(S):-
    string_lower(S, Lower),
    string_chars(Lower, LowerChars),
    include(alpha, LowerChars, LowerAlpha),
    sort(LowerAlpha, B),
    msort(LowerAlpha, C),
    equal_len(B, C).
    
alpha(X):-
    char_type(X, alnum).

equal_len(A, B):-
    length(A, X),
    length(B, X).