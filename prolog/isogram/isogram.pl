only_alpha([Char | CharsIn], [Char | CharsOut]) :-
    Char @>= 'a', Char @=< 'z', !,
    only_alpha(CharsIn, CharsOut).
only_alpha([_ | CharsIn], CharsOut) :-
    only_alpha(CharsIn, CharsOut).
only_alpha([], []).

isogram(Phrase):-
    string_lower(Phrase, Lower),
    string_chars(Lower, LowerChars),
    only_alpha(LowerChars, LowerAlpha),
    sort(LowerAlpha, B),
    sort(0, @=<, LowerAlpha, C),
    length(B, LenB),
    length(C, LenC),
    LenB == LenC.
    
