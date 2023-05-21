:- use_module(library(clpfd)).

binary(BsString, N) :-
    string_chars(BsString, Chars),
    maplist(char_to_num, Chars, Bs),
    binary_number_min(Bs, 0, N, N).

binary_number_min([], N, N, _M).
binary_number_min([B|Bs], N0, N, M) :-
    B in 0..1,
    N1 #= B+2*N0,
    M #>= N1,
    binary_number_min(Bs, N1, N, M).

char_to_num(C, Num):-
    atom_number(C, Num).